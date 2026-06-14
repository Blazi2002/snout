import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface SearchResult {
  path: string;
  score: number;
  preview: string;
}

interface IndexSummary {
  added: number;
  updated: number;
  unchanged: number;
}

interface IndexProgress {
  processed: number;
  total: number;
}

function App() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [searched, setSearched] = useState(false);
  const [indexing, setIndexing] = useState(false);
  const [status, setStatus] = useState("");
  const [progress, setProgress] = useState<IndexProgress | null>(null);

  // Ascolta gli eventi di progresso emessi dal backend durante l'indicizzazione.
  useEffect(() => {
    const unlisten = listen<IndexProgress>("index-progress", (event) => {
      setProgress(event.payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  async function handleChooseAndIndex() {
    try {
      const selected = await open({ directory: true, multiple: false });
      if (!selected || typeof selected !== "string") return;

      setIndexing(true);
      setProgress(null);
      setStatus("");
      setError("");

      const summary = await invoke<IndexSummary>("index_folder", { path: selected });
      setStatus(
        `Done: ${summary.added} added, ${summary.updated} updated, ${summary.unchanged} unchanged.`
      );
      setProgress(null);
    } catch (e) {
      setError(String(e));
      setStatus("");
      setProgress(null);
    } finally {
      setIndexing(false);
    }
  }

  async function handleSearch() {
    if (query.trim() === "") {
      setResults([]);
      setSearched(false);
      return;
    }

    setLoading(true);
    setError("");
    setSearched(true);

    try {
      const hits = await invoke<SearchResult[]>("search_files", { query });
      setResults(hits);
    } catch (e) {
      setError(String(e));
      setResults([]);
    } finally {
      setLoading(false);
    }
  }

  async function handleOpenFile(path: string) {
    try {
      await invoke("open_file", { path });
    } catch (e) {
      setError(`Could not open file: ${e}`);
    }
  }

  function splitPath(full: string): { name: string; dir: string } {
    const parts = full.split("/");
    const name = parts[parts.length - 1] || full;
    const dir = parts.slice(0, -1).join("/");
    return { name, dir };
  }

  const progressPercent =
    progress && progress.total > 0
      ? Math.round((progress.processed / progress.total) * 100)
      : 0;

  return (
    <div className="app">
      <header className="header">
        <div className="logo">
          <span className="logo-mark">🐾</span>
          <span className="logo-text">Snout</span>
        </div>
        <p className="tagline">Search your files by meaning, locally.</p>
      </header>

      <div className="toolbar">
        <button className="index-button" onClick={handleChooseAndIndex} disabled={indexing}>
          {indexing ? "Indexing..." : "📁 Choose folder & index"}
        </button>
        {status && <span className="status">{status}</span>}
      </div>

      {indexing && progress && (
        <div className="progress-wrap">
          <div className="progress-bar">
            <div className="progress-fill" style={{ width: `${progressPercent}%` }} />
          </div>
          <span className="progress-text">
            {progress.processed} / {progress.total} files ({progressPercent}%)
          </span>
        </div>
      )}

      <div className="search-bar">
        <span className="search-icon">⌕</span>
        <input
          className="search-input"
          type="text"
          placeholder="Search for anything — a word, a topic, an idea..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          onKeyDown={(e) => { if (e.key === "Enter") handleSearch(); }}
          autoFocus
        />
        <button className="search-button" onClick={handleSearch} disabled={loading}>
          {loading ? "..." : "Search"}
        </button>
      </div>

      <div className="results">
        {error && <div className="error-state">{error}</div>}

        {!error && loading && (
          <div className="empty-state"><p>Searching...</p></div>
        )}

        {!error && !loading && searched && results.length === 0 && (
          <div className="empty-state"><p>No results found.</p></div>
        )}

        {!error && !loading && !searched && (
          <div className="empty-state">
            <p>Choose a folder to index, then search your files.</p>
          </div>
        )}

        {!error && !loading && results.map((r, i) => {
          const { name, dir } = splitPath(r.path);
          return (
            <div className="result-card" key={i} onClick={() => handleOpenFile(r.path)}>
              <div className="result-header">
                <span className="result-name">{name}</span>
                <span className="result-score">{(r.score * 1000).toFixed(1)}</span>
              </div>
              <p className="result-dir">{dir}</p>
              {r.preview && <p className="result-preview">{r.preview}</p>}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default App;

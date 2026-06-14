import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// Struttura di un risultato, come arriva dal motore Rust.
interface SearchResult {
  path: string;
  score: number;
}

function App() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [searched, setSearched] = useState(false);

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
      // Chiamiamo il command Rust 'search_files' passandogli la query.
      const hits = await invoke<SearchResult[]>("search_files", { query });
      setResults(hits);
    } catch (e) {
      setError(String(e));
      setResults([]);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="app">
      <header className="header">
        <div className="logo">
          <span className="logo-mark">🐾</span>
          <span className="logo-text">Snout</span>
        </div>
        <p className="tagline">Search your files by meaning, locally.</p>
      </header>

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
            <p>Type something and press Enter to search your files.</p>
          </div>
        )}

        {!error && !loading && results.map((r, i) => (
          <div className="result-card" key={i}>
            <div className="result-header">
              <span className="result-path">{r.path}</span>
              <span className="result-score">{(r.score * 1000).toFixed(1)}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;

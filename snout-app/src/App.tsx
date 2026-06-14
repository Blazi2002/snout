import { useState } from "react";
import "./App.css";

// Risultati finti di esempio, solo per costruire e vedere il design.
// Verranno sostituiti dai risultati reali del motore di ricerca.
const MOCK_RESULTS = [
  { path: "Documenti/ricette/carbonara.docx", snippet: "La ricetta della carbonara prevede uova, guanciale, pecorino e pepe nero...", score: 0.94 },
  { path: "Documenti/finanza/investimenti_2025.pdf", snippet: "Per investire in borsa è importante diversificare il portafoglio...", score: 0.81 },
  { path: "Documenti/casa/animali_domestici.txt", snippet: "Il gatto e il cane sono animali domestici molto diffusi nelle case italiane...", score: 0.76 },
];

function App() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<typeof MOCK_RESULTS>([]);

  function handleSearch() {
    // Per ora mostriamo i risultati finti. Qui collegheremo il motore reale.
    if (query.trim() === "") {
      setResults([]);
      return;
    }
    setResults(MOCK_RESULTS);
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
        <button className="search-button" onClick={handleSearch}>
          Search
        </button>
      </div>

      <div className="results">
        {results.length === 0 ? (
          <div className="empty-state">
            <p>Type something and press Enter to search your files.</p>
          </div>
        ) : (
          results.map((r, i) => (
            <div className="result-card" key={i}>
              <div className="result-header">
                <span className="result-path">{r.path}</span>
                <span className="result-score">{Math.round(r.score * 100)}%</span>
              </div>
              <p className="result-snippet">{r.snippet}</p>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

export default App;

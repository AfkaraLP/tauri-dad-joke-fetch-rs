import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [dadJoke, setDadJoke] = useState<string>("");

  async function getDadJoke(): Promise<void> {
    const joke = await invoke<string>("get_dad_joke");
    setDadJoke(joke);
  }

  useEffect(() => {
    getDadJoke();
  }, []);

  return (
    <main className="app-container">
      <h1 className="title">Dad Joke Fetcher</h1>
      
      <p className="subtitle">
        Tauri Version.
      </p>

      <button className="neobrutal-btn" onClick={getDadJoke} type="button">
        Get Another Joke
      </button>

      {dadJoke && <div className="dad-joke">{dadJoke}</div>}
    </main>
  );
}

export default App;

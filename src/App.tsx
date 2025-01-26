import { useEffect } from "react"
import { invoke } from "@tauri-apps/api/core"
import "./App.css"

export default function App() {
  useEffect(() => {
    async function handleKeyPress() {
      await invoke("play_sound")
    }

    window.addEventListener("keypress", handleKeyPress)

    return () => {
      window.removeEventListener("keypress", handleKeyPress)
    }
  }, [])

  return (
    <main className="container">
      <h1>Hello</h1>
    </main>
  )
}

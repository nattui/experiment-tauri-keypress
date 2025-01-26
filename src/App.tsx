import { useEffect } from "react"
import "./App.css"

export default function App() {
  useEffect(() => {
    const handleKeyPress = () => {
      // Create and play a new audio instance for each keypress
      const audio = new Audio("/sound.wav")
      audio.play()
      // Clean up the audio element after it finishes playing
      audio.addEventListener("ended", () => {
        audio.remove()
      })
    }

    window.addEventListener("keydown", handleKeyPress)

    return () => {
      window.removeEventListener("keydown", handleKeyPress)
    }
  }, [])

  return (
    <main className="container">
      <h1>Hello</h1>
    </main>
  )
}

import { useState, useEffect, useCallback, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

const PRESETS = [5, 15, 25, 45];
const RADIUS = 88;
const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

function formatTime(totalSeconds: number) {
  const m = Math.floor(totalSeconds / 60).toString().padStart(2, "0");
  const s = (totalSeconds % 60).toString().padStart(2, "0");
  return `${m}:${s}`;
}

type TickPayload = {
  seconds_left: number;
  total_seconds: number;
  running: boolean;
};

function App() {
  const [secondsLeft, setSecondsLeft] = useState(25 * 60);
  const [totalSeconds, setTotalSeconds] = useState(25 * 60);
  const [isRunning, setIsRunning] = useState(false);

  useEffect(() => {
    invoke<TickPayload>("get_timer_state").then((data) => {
      setSecondsLeft(data.seconds_left);
      setTotalSeconds(data.total_seconds);
      setIsRunning(data.running);
    });

    const unlisten = listen<TickPayload>("timer-tick", (event) => {
      setSecondsLeft(event.payload.seconds_left);
      setTotalSeconds(event.payload.total_seconds);
      setIsRunning(event.payload.running);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const minutes = Math.round(totalSeconds / 60);

  const handlePreset = useCallback((mins: number) => {
    invoke("set_minutes", { minutes: mins });
  }, []);

  const handleSliderChange = useCallback((mins: number) => {
    invoke("set_minutes", { minutes: mins });
  }, []);

  const handleReset = useCallback(() => {
    invoke("reset_timer");
  }, []);

  const handleToggle = useCallback(() => {
    if (isRunning) {
      invoke("pause_timer");
    } else {
      if (secondsLeft <= 0) return;
      invoke("start_timer");
    }
  }, [isRunning, secondsLeft]);

  const progress = totalSeconds > 0 ? secondsLeft / totalSeconds : 0;
  const isUrgent = progress <= 0.2 && progress > 0;
  const dashOffset = useMemo(() => CIRCUMFERENCE * (1 - progress), [progress]);

  return (
    <div className="container">
      <div className={`ring-wrap ${isRunning ? "running" : ""}`}>
        <svg width="200" height="200" viewBox="0 0 200 200">
          <circle className="ring-track" cx="100" cy="100" r={RADIUS} fill="none" strokeWidth="4" />
          <circle
            className={`ring-progress ${isUrgent ? "urgent" : ""}`}
            cx="100"
            cy="100"
            r={RADIUS}
            fill="none"
            strokeWidth="4"
            strokeDasharray={CIRCUMFERENCE}
            strokeDashoffset={dashOffset}
            strokeLinecap="round"
            transform="rotate(-90 100 100)"
          />
        </svg>
        <div className="time-display">{formatTime(secondsLeft)}</div>
      </div>

      <input
        type="range"
        min={1}
        max={60}
        value={minutes}
        disabled={isRunning}
        onChange={(e) => handleSliderChange(Number(e.target.value))}
        className="slider"
      />
      <div className="minutes-label">{minutes} min</div>

      <div className="presets">
        {PRESETS.map((p) => (
          <button
            key={p}
            className={`preset-btn ${minutes === p ? "active" : ""}`}
            onClick={() => handlePreset(p)}
            disabled={isRunning}
          >
            {p}
          </button>
        ))}
      </div>

      <div className="controls">
        <button className="control-btn" onClick={handleReset}>
          Reset
        </button>
        <button className="control-btn primary" onClick={handleToggle}>
          {isRunning ? "Pause" : "Start"}
        </button>
      </div>
    </div>
  );
}

export default App;
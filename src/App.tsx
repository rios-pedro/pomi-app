import { useState, useEffect, useCallback, useRef, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import "./App.css";

const PRESETS = [5, 15, 25, 45];
const RADIUS = 88;
const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

function formatTime(totalSeconds: number) {
  const m = Math.floor(totalSeconds / 60)
    .toString()
    .padStart(2, "0");
  const s = (totalSeconds % 60).toString().padStart(2, "0");
  return `${m}:${s}`;
}

async function notifyDone() {
  let granted = await isPermissionGranted();
  if (!granted) {
    const permission = await requestPermission();
    granted = permission === "granted";
  }
  if (granted) {
    sendNotification({ title: "Pomi", body: "Tempo esgotado!" });
  }
}

function App() {
  const [minutes, setMinutes] = useState(25);
  const [secondsLeft, setSecondsLeft] = useState(25 * 60);
  const [isRunning, setIsRunning] = useState(false);
  const hasNotified = useRef(false);

  useEffect(() => {
    if (!isRunning) setSecondsLeft(minutes * 60);
  }, [minutes]);

  useEffect(() => {
    invoke("update_tray_title", { time: formatTime(secondsLeft) });
  }, [secondsLeft]);

  useEffect(() => {
    if (!isRunning) return;

    if (secondsLeft <= 0) {
      setIsRunning(false);
      if (!hasNotified.current) {
        hasNotified.current = true;
        notifyDone();
      }
      return;
    }

    const interval = setInterval(() => {
      setSecondsLeft((prev) => prev - 1);
    }, 1000);

    return () => clearInterval(interval);
  }, [isRunning, secondsLeft]);

  const handlePreset = useCallback((mins: number) => {
    setIsRunning(false);
    hasNotified.current = false;
    setMinutes(mins);
    setSecondsLeft(mins * 60);
  }, []);

  const handleReset = useCallback(() => {
    setIsRunning(false);
    hasNotified.current = false;
    setSecondsLeft(minutes * 60);
  }, [minutes]);

  const handleToggle = useCallback(() => {
    if (secondsLeft <= 0) return;
    hasNotified.current = false;
    setIsRunning((r) => !r);
  }, [secondsLeft]);

  const totalSeconds = minutes * 60;
  const progress = totalSeconds > 0 ? secondsLeft / totalSeconds : 0;
  const isUrgent = progress <= 0.2 && progress > 0;
  const dashOffset = useMemo(
    () => CIRCUMFERENCE * (1 - progress),
    [progress]
  );

  return (
    <div className="container">
      <div className={`ring-wrap ${isRunning ? "running" : ""}`}>
        <svg width="200" height="200" viewBox="0 0 200 200">
          <circle
            className="ring-track"
            cx="100"
            cy="100"
            r={RADIUS}
            fill="none"
            strokeWidth="4"
          />
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
        onChange={(e) => setMinutes(Number(e.target.value))}
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
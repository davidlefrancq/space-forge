import React, { useState, useEffect } from "react"

type TimeUnit = "day" | "month" | "year" | "century" | "millennium"

interface TimeStepSelectorProps {
  initialValue?: number
  initialUnit?: TimeUnit
  onChange?: (seconds: number) => void
}

const TimeStepSelector: React.FC<TimeStepSelectorProps> = ({ initialValue = 1, initialUnit = "day", onChange }) => {
  const [amount, setAmount] = useState(initialValue)
  const [unit, setUnit] = useState<TimeUnit>(initialUnit)

  const computeSeconds = (value: number, unit: TimeUnit): number => {
    const secondsInDay = 24 * 60 * 60
    switch (unit) {
      case "day":
        return value * secondsInDay
      case "month":
        return value * 30 * secondsInDay
      case "year":
        return value * 365 * secondsInDay
      case "century":
        return value * 100 * 365 * secondsInDay
      case "millennium":
        return value * 1000 * 365 * secondsInDay
      default:
        return value * secondsInDay
    }
  }

  useEffect(() => {
    const result = computeSeconds(amount, unit)
    if (onChange) onChange(result)
  }, [amount, unit])

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
      <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
        <input
          type="number"
          step="1"
          min="1"
          value={amount}
          onChange={(e) => setAmount(parseFloat(e.target.value))}
          className="w-20 px-2 py-1 bg-zinc-800 text-white border border-zinc-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
        />
        <select
          value={unit}
          onChange={(e) => setUnit(e.target.value as TimeUnit)}
          className="px-2 py-1 bg-zinc-800 text-white border border-zinc-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
        >
          <option value="day">jour(s)</option>
          <option value="month">mois</option>
          <option value="year">année(s)</option>
          <option value="century">siècle(s)</option>
          <option value="millennium">millénaire(s)</option>
        </select>
      </div>
    </div>
  )
}

export default TimeStepSelector

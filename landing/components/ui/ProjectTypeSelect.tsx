"use client";

import { useState, useRef, useEffect } from "react";
import { PROJECT_TYPES } from "@/data/contact";

function ChevronDownIcon({
  className = "h-3.5 w-3.5",
}: {
  className?: string;
}) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      strokeWidth={1.5}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M19.5 8.25l-7.5 7.5-7.5-7.5"
      />
    </svg>
  );
}

type ProjectTypeValue = (typeof PROJECT_TYPES)[number]["value"];

export default function ProjectTypeSelect({
  value,
  onChange,
  onBlur,
  error,
  disabled,
}: {
  value?: ProjectTypeValue;
  onChange: (value: ProjectTypeValue) => void;
  onBlur?: () => void;
  error?: string;
  disabled?: boolean;
}) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const selected = PROJECT_TYPES.find((t) => t.value === value);

  return (
    <div ref={ref}>
      <div
        className={`flex items-center font-poppins border-b bg-transparent transition-colors  focus-within:border-white/50 ${
          error ? "border-red-400/60" : "border-white/12"
        } ${disabled ? "cursor-not-allowed opacity-50" : ""}`}
      >
        <button
          type="button"
          onClick={() => {
            if (disabled) return;
            setOpen((prev) => !prev);
            if (open) onBlur?.();
          }}
          disabled={disabled}
          className="flex w-full items-center cursor-pointer justify-between py-3 font-poppins text-[11px] text-light-gray outline-none disabled:cursor-not-allowed sm:text-[13px]"
        >
          <span className={selected ? "text-light-gray" : "text-white/40"}>
            {selected ? selected.label : "Select a project type"}
          </span>
          <ChevronDownIcon
            className={`h-3.5 w-3.5 shrink-0 text-white/70 transition-transform ${open ? "rotate-180" : ""}`}
          />
        </button>
      </div>

      {open && !disabled && (
        <div className="relative">
          <div className="absolute z-50 mt-1 w-full overflow-hidden rounded-xl border border-white/10 bg-[#0b0b0b] shadow-[0_15px_50px_rgba(0,0,0,0.5)] backdrop-blur-xl">
            {PROJECT_TYPES.map((type) => (
              <button
                key={type.value}
                type="button"
                onClick={() => {
                  onChange(type.value);
                  setOpen(false);
                  onBlur?.();
                }}
                className={`flex w-full items-center font-poppins cursor-pointer justify-between px-3 py-2.5 text-left text-xs transition-colors hover:bg-white/6 ${
                  value === type.value
                    ? "bg-white/8 font-medium text-white"
                    : "text-white/80"
                }`}
              >
                <span>{type.label}</span>
              </button>
            ))}
          </div>
        </div>
      )}

      {error && (
        <p className="mt-1.5 font-poppins text-[11px] text-red-400/80">
          {error}
        </p>
      )}
    </div>
  );
}

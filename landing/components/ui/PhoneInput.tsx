"use client";

import { useState, useRef, useEffect } from "react";
import {
  AsYouType,
  getCountries,
  getCountryCallingCode,
  validatePhoneNumberLength,
} from "libphonenumber-js";
import { PHONE_CODES } from "@/data/phone";

// Inline SVG Icons
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

function SearchIcon({ className = "h-3.5 w-3.5" }: { className?: string }) {
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
        d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
      />
    </svg>
  );
}

function isoForDialCode(dialCode: string) {
  const digits = dialCode.replace("+", "");
  return getCountries().find((iso) => getCountryCallingCode(iso) === digits);
}

export default function PhoneInput({
  phoneCode,
  setPhoneCode,
  phoneNumber,
  setPhoneNumber,
  onBlur,
  error,
  disabled,
}: {
  phoneCode: string;
  setPhoneCode: (code: string) => void;
  phoneNumber?: string;
  setPhoneNumber?: (value: string) => void;
  onBlur?: () => void;
  error?: string;
  disabled?: boolean;
}) {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
        setSearch("");
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const filtered = PHONE_CODES.filter(
    (pc) =>
      pc.code.includes(search) ||
      pc.country.toLowerCase().includes(search.toLowerCase()),
  );
  const selected = PHONE_CODES.find((pc) => pc.code === phoneCode);

  const handleNumberChange = (raw: string) => {
    const digits = raw.replace(/\D/g, "");

    if (
      digits &&
      validatePhoneNumberLength(`${phoneCode}${digits}`) === "TOO_LONG"
    ) {
      return;
    }

    const iso = isoForDialCode(phoneCode);
    const formatted = iso ? new AsYouType(iso as never).input(digits) : digits;
    setPhoneNumber?.(formatted);
  };

  return (
    <div>
      <div
        className={`flex items-center font-poppins gap-2.5 border-b bg-transparent transition-colors focus-within:border-white/50 ${
          error ? "border-red-400/60" : "border-white/12"
        } ${disabled ? "cursor-not-allowed opacity-50" : ""}`}
      >
        {/* Country code dropdown */}
        <div className="relative shrink-0" ref={ref}>
          <button
            type="button"
            onClick={() => !disabled && setOpen(!open)}
            disabled={disabled}
            className="flex items-center gap-1 py-3 font-poppins text-[11px] sm:text-[13px] text-light-gray outline-none disabled:cursor-not-allowed"
          >
            <span>{selected ? selected.code : "Code"}</span>
            <ChevronDownIcon className="h-3.5 w-3.5 text-white/70" />
          </button>

          {open && !disabled && (
            <div className="absolute z-50 mt-1 w-56 overflow-hidden rounded-xl border border-white/10 bg-[#0b0b0b] shadow-[0_15px_50px_rgba(0,0,0,0.5)] backdrop-blur-xl sm:w-64">
              <div className="relative border-b border-white/8">
                <SearchIcon className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-white/70" />
                <input
                  type="text"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  placeholder="Search country or code..."
                  className="w-full bg-transparent py-2 pl-9 pr-4 font-poppins text-[10px] sm:text-[12px] text-white/80 outline-none placeholder:text-white/40 sm:py-2.5"
                />
              </div>

              <div className="max-h-48 overflow-y-auto scrollbar-none [-ms-overflow-style:none] [&::-webkit-scrollbar]:hidden">
                {filtered.map((pc) => (
                  <button
                    key={pc.country}
                    type="button"
                    onClick={() => {
                      setPhoneCode(pc.code);
                      setOpen(false);
                      setSearch("");
                    }}
                    className={`flex w-full items-center justify-between cursor-pointer px-3 py-2 text-left text-xs transition-colors hover:bg-white/6 ${
                      phoneCode === pc.code
                        ? "bg-white/8 font-medium text-white"
                        : "text-white/80"
                    }`}
                  >
                    <span>{pc.country}</span>
                    <span className="text-white/70">{pc.code}</span>
                  </button>
                ))}
                {filtered.length === 0 && (
                  <p className="px-4 py-3 text-xs text-white/70">
                    No results found
                  </p>
                )}
              </div>
            </div>
          )}
        </div>

        <span className="h-4 w-px bg-white/12" />

        <input
          type="tel"
          inputMode="numeric"
          placeholder="Phone number"
          value={phoneNumber}
          onChange={(e) => handleNumberChange(e.target.value)}
          onBlur={onBlur}
          disabled={disabled}
          className="flex-1 bg-transparent py-3 font-poppins text-[11px] sm:text-[13px] text-light-gray outline-none placeholder:text-white/20 disabled:cursor-not-allowed"
        />
      </div>

      {error && (
        <p className="mt-1.5 font-poppins text-[11px] text-red-400/80">
          {error}
        </p>
      )}
    </div>
  );
}

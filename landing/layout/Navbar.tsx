"use client";

import { useState, useEffect, useCallback } from "react";
import { NAVLINKS } from "@/data/navigation";
import Link from "next/link";

export default function Navbar() {
  const [scrolled, setScrolled] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const handleScroll = useCallback(() => {
    setScrolled(window.scrollY > 10);
  }, []);

  useEffect(() => {
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  }, [handleScroll]);

  useEffect(() => {
    const lock = mobileMenuOpen ? "hidden" : "";
    document.documentElement.style.overflow = lock;
    document.body.style.overflow = lock;

    return () => {
      document.documentElement.style.overflow = "";
      document.body.style.overflow = "";
    };
  }, [mobileMenuOpen]);

  useEffect(() => {
    if (!mobileMenuOpen) return;

    const closeMenu = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (
        !target.closest(".mobile-menu") &&
        !target.closest(".mobile-toggle")
      ) {
        setMobileMenuOpen(false);
      }
    };

    document.addEventListener("mousedown", closeMenu);
    return () => document.removeEventListener("mousedown", closeMenu);
  }, [mobileMenuOpen]);

  return (
    <nav
      className={`fixed w-full top-0 left-0 z-50 transition-all duration-300 font-poppins ${
        mobileMenuOpen
          ? "bg-black/50"
          : scrolled
            ? "bg-black/50 backdrop-blur-md"
            : "bg-transparent"
      }`}
    >
      <div className="px-6 sm:px-12 lg:px-16 py-5 sm:py-6 flex items-center justify-between">
        <Link href="/">
          <span className="text-lg sm:text-xl font-bold tracking-tight text-text z-50">
            atronsa.
          </span>
        </Link>

        <div className="flex items-center gap-4">
          <button
            className="mobile-toggle z-50 p-2 text-text cursor-pointer"
            onClick={() => setMobileMenuOpen((prev) => !prev)}
            aria-label="Toggle mobile menu"
            aria-expanded={mobileMenuOpen}
            aria-controls="mobile-menu"
          >
            <div className="w-6 h-4 sm:w-8 sm:h-6 md:w-8 md:h-6 flex flex-col justify-center items-center gap-1.5">
              <span
                className={`block w-full h-0.5 bg-current transition-all ${
                  mobileMenuOpen ? "rotate-45 translate-y-2" : ""
                }`}
              />
              <span
                className={`block w-full h-0.5 bg-current transition-all ${
                  mobileMenuOpen ? "opacity-0" : ""
                }`}
              />
              <span
                className={`block w-full h-0.5 bg-current transition-all ${
                  mobileMenuOpen ? "-rotate-45 -translate-y-2" : ""
                }`}
              />
            </div>
          </button>
        </div>
      </div>

      {mobileMenuOpen && (
        <div
          id="mobile-menu"
          role="dialog"
          aria-modal="true"
          className="mobile-menu fixed inset-0 z-40 flex items-center justify-center bg-black/50 backdrop-blur-md"
        >
          <ul className="space-y-6 text-center">
            {NAVLINKS.map(({ link, name }) => (
              <li key={name}>
                <a
                  href={link}
                  className="cursor-pointer relative inline-block text-sm md:text-sm lg:text-base px-6 py-2 sm:py-3 md:py-3 after:absolute after:left-1/2 after:bottom-0 after:h-0.5 after:w-0 after:bg-current after:transition-all after:duration-300 hover:after:w-full hover:after:left-0"
                  onClick={() => setMobileMenuOpen(false)}
                >
                  {name}
                </a>
              </li>
            ))}
          </ul>
        </div>
      )}
    </nav>
  );
}

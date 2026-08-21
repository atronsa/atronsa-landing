import Link from "next/link";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "404 — Page Not Found",
  description: "The page you're looking for doesn't exist.",
  robots: {
    index: false,
    follow: false,
  },
};

export default function NotFound() {
  return (
    <main className="flex min-h-screen items-center justify-center bg-black px-6 text-white">
      <div className="flex w-full max-w-xl flex-col items-center text-center">
        <h1 className="select-none font-title text-[clamp(8rem,22vw,15rem)] font-medium leading-[0.75] tracking-[-0.09em] text-white">
          404
        </h1>

        <div className="mt-10 h-px w-10 bg-white/20" />

        <div className="mt-7">
          <h2 className="font-title text-xl font-medium tracking-[-0.02em] sm:text-2xl">
            This page doesn't exist.
          </h2>

          <p className="mx-auto mt-3 max-w-md font-poppins text-xs sm:text-sm leading-relaxed text-white/40">
            The page you're looking for may have been moved, deleted, or never
            existed in the first place.
          </p>
        </div>

        <Link
          href="/"
          className="mt-8 inline-flex items-center gap-2 border-b border-white/30 pb-1 font-poppins text-xs text-white/70 transition-colors duration-300 hover:border-white hover:text-white sm:text-sm cursor-pointer"
        >
          <svg
            className="h-3.5 w-3.5"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="1.5"
          >
            <path d="M19 12H5" />
            <path d="M11 18l-6-6 6-6" />
          </svg>
          Back to home
        </Link>
      </div>
    </main>
  );
}

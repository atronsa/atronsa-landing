"use client";

import { STARS } from "@/data/star";

export default function Hero() {
  return (
    <section className="relative flex min-h-screen w-full items-center justify-center overflow-hidden bg-black px-4 py-16 sm:px-6 md:px-8">
      <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_center,rgba(255,255,255,0.03)_0%,transparent_60%)]" />
      <div
        aria-hidden="true"
        className="pointer-events-none absolute inset-0"
        style={{
          backgroundImage:
            "linear-gradient(to right, rgba(255,255,255,0.03) 1px, transparent 1px), linear-gradient(to bottom, rgba(255,255,255,0.03) 1px, transparent 1px)",
          backgroundSize: "80px 80px",
          maskImage:
            "radial-gradient(ellipse 60% 50% at 50% 50%, black 30%, transparent 100%)",
          WebkitMaskImage:
            "radial-gradient(ellipse 60% 50% at 50% 50%, black 30%, transparent 100%)",
        }}
      />
      <div aria-hidden="true" className="pointer-events-none absolute inset-0">
        {STARS.map((star, i) => (
          <span
            key={i}
            className="absolute animate-twinkle rounded-full bg-white"
            style={{
              top: star.top,
              left: star.left,
              width: star.size,
              height: star.size,
              opacity: star.opacity * 0.4,
              animationDelay: `${star.delay}s`,
              boxShadow: `0 0 ${star.size * 2}px rgba(255,255,255,${
                star.opacity * 0.3
              })`,
            }}
          />
        ))}
      </div>

      <div className="relative z-10 flex w-full max-w-[1600px] flex-col items-center px-0 text-center sm:px-4">
        <h1 className="animate-fade-in-up select-none whitespace-nowrap font-title font-bold tracking-tight leading-[0.8] text-white [animation-delay:200ms] text-[clamp(4rem,18vw,13rem)]">
          Atronsa
        </h1>

        <p className="animate-fade-in-up mx-auto w-full max-w-xl pt-6 font-poppins text-sm leading-6 text-light-gray [animation-delay:300ms] sm:max-w-[90%] sm:pt-8 sm:text-base sm:leading-7 md:max-w-2xl md:pt-10 lg:max-w-4xl">
          We build digital experiences that drive growth and deliver real
          results. Your vision, our expertise — let's create something
          extraordinary.
        </p>

        <div className="animate-fade-in-up mt-10 flex w-full flex-col items-center justify-center gap-3 px-6 font-poppins [animation-delay:450ms] sm:mt-12 sm:w-auto sm:flex-row sm:gap-4 sm:px-0">
          <button
            type="button"
            className="group relative inline-block w-full max-w-[320px] cursor-pointer rounded-xl bg-white/20 p-px text-sm leading-6 text-white no-underline shadow-2xl shadow-black/10 transition-all duration-300 hover:bg-white/30 sm:w-auto sm:max-w-none"
            onClick={() => {
              const contactSection = document.getElementById("contact");
              contactSection?.scrollIntoView({ behavior: "smooth" });
            }}
          >
            <span className="absolute inset-0 overflow-hidden rounded-xl">
              <span className="absolute inset-0 rounded-full bg-[radial-gradient(75%_100%_at_50%_0%,rgba(255,255,255,0.4)_0%,rgba(255,255,255,0)_75%)] opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
            </span>

            <span className="relative z-10 flex items-center justify-center gap-2 rounded-xl bg-black/50 px-7 py-3 backdrop-blur-md ring-1 ring-white/20 transition-colors">
              <span className="text-xs sm:text-sm">Start your project</span>
              <svg
                aria-hidden="true"
                className="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M13 7l5 5m0 0l-5 5m5-5H6"
                />
              </svg>
            </span>

            <span className="absolute bottom-0 left-4.5 h-px w-[calc(100%-2.25rem)] bg-linear-to-r from-white/0 via-white/90 to-white/0" />
          </button>

          <button
            type="button"
            className="group relative inline-block w-full max-w-[320px] cursor-pointer rounded-xl bg-white/20 p-px text-sm leading-6 text-white no-underline shadow-2xl shadow-black/10 transition-all duration-300 hover:bg-white/30 sm:w-auto sm:max-w-none"
            onClick={() => {
              const contactSection = document.getElementById("work");
              contactSection?.scrollIntoView({ behavior: "smooth" });
            }}
          >
            <span className="absolute inset-0 overflow-hidden rounded-xl">
              <span className="absolute inset-0 rounded-full bg-[radial-gradient(75%_100%_at_50%_0%,rgba(255,255,255,0.2)_0%,rgba(255,255,255,0)_75%)] opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
            </span>

            <span className="relative z-10 flex items-center justify-center gap-2 rounded-xl bg-black/50 px-7 py-3 backdrop-blur-sm ring-1 ring-white/20">
              <span className="text-xs sm:text-sm">View our work</span>
              <svg
                aria-hidden="true"
                className="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M13 7l5 5m0 0l-5 5m5-5H6"
                />
              </svg>
            </span>

            <span className="absolute bottom-0 left-4.5 h-px w-[calc(100%-2.25rem)] bg-linear-to-r from-white/0 via-white/50 to-white/0 transition-opacity" />
          </button>
        </div>
      </div>
    </section>
  );
}

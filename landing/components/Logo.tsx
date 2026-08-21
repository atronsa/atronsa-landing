"use client";

import { logos } from "@/data/logo";

export default function Logo() {
  return (
    <section className="relative w-full overflow-hidden bg-black pt-10 sm:pt-12">
      <div className="relative w-full">
        <div
          aria-hidden="true"
          className="pointer-events-none absolute inset-y-0 left-0 z-20 w-20 bg-linear-to-r from-black to-transparent sm:w-28 md:w-40"
        />
        <div
          aria-hidden="true"
          className="pointer-events-none absolute inset-y-0 right-0 z-20 w-20 bg-linear-to-l from-black to-transparent sm:w-28 md:w-40"
        />

        <style jsx>{`
          @keyframes marquee {
            from {
              transform: translateX(0);
            }
            to {
              transform: translateX(-50%);
            }
          }
          .marquee-track {
            animation: marquee 30s linear infinite;
            will-change: transform;
          }
          @media (prefers-reduced-motion: reduce) {
            .marquee-track {
              animation: none;
            }
          }
        `}</style>

        <div className="marquee-track flex w-max">
          <div className="flex shrink-0 items-center gap-10 pr-10 sm:gap-14 sm:pr-14 md:gap-20 md:pr-20 lg:gap-24 lg:pr-24">
            {logos.map(({ id, svg }) => (
              <div
                key={`first-${id}`}
                className="flex h-14 w-20 shrink-0 items-center justify-center opacity-80 sm:h-16 sm:w-24 md:h-20 md:w-28"
              >
                {svg}
              </div>
            ))}
          </div>

          <div
            aria-hidden="true"
            className="flex shrink-0 items-center gap-10 pr-10 sm:gap-14 sm:pr-14 md:gap-20 md:pr-20 lg:gap-24 lg:pr-24"
          >
            {logos.map(({ id, svg }) => (
              <div
                key={`second-${id}`}
                className="flex h-14 w-20 shrink-0 items-center justify-center opacity-80 sm:h-16 sm:w-24 md:h-20 md:w-28"
              >
                {svg}
              </div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
}

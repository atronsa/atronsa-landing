"use client";

import { useRef, useEffect } from "react";
import gsap from "gsap";
import { logos } from "@/data/Logo";

export default function Logo() {
  const trackRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!trackRef.current) return;

    const track = trackRef.current;

    const animation = gsap.to(track, {
      xPercent: -50,
      duration: 30,
      ease: "none",
      repeat: -1,
    });

    return () => {
      animation.kill();
    };
  }, []);

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
        <div ref={trackRef} className="flex w-max will-change-transform">
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

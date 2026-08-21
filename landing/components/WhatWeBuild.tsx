import Link from "next/link";
import { PROJECTS } from "@/data/project";

export default function WhatWeBuild() {
  return (
    <section
      id="work"
      className="relative overflow-hidden bg-black px-4 sm:px-6 lg:px-16 pt-10 sm:pt-12 md:pt-16 lg:pt-24"
    >
      <div
        className="pointer-events-none absolute inset-0 opacity-30"
        style={{
          backgroundImage: `
            linear-gradient(rgba(255,255,255,0.025) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.025) 1px, transparent 1px)
          `,
          backgroundSize: "80px 80px",
          maskImage:
            "linear-gradient(to bottom, transparent, black 15%, black 85%, transparent)",
          WebkitMaskImage:
            "linear-gradient(to bottom, transparent, black 15%, black 85%, transparent)",
        }}
      />

      <div className="relative mx-auto max-w-350">
        <div className="mx-auto max-w-3xl text-center">
          <div className="mb-5 inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/[0.035] px-3.5 py-1.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)] backdrop-blur-xl sm:mb-7 sm:px-4">
            <span className="relative flex h-1.5 w-1.5">
              <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-white/30" />
              <span className="relative inline-flex h-1.5 w-1.5 rounded-full bg-white/70" />
            </span>

            <span className="font-poppins text-[9px] tracking-wide text-white/50 sm:text-xs">
              Project
            </span>
          </div>
        </div>
        <div className="mb-12 text-center sm:mb-16">
          <h2 className="font-title text-[clamp(3rem,7vw,6rem)] font-medium leading-[0.9] tracking-[-0.055em] text-white">
            What We Build
          </h2>
        </div>

        <div className="flex flex-col items-center gap-6 md:grid md:grid-cols-2 md:gap-3">
          {PROJECTS.map((project) => (
            <div
              key={project.title}
              className={`${project.className} group relative w-full max-w-md sm:max-w-none min-h-105 overflow-hidden rounded-xl border border-white/8 bg-[#111] sm:min-h-125 md:min-h-130 lg:min-h-140`}
            >
              <div
                className="absolute inset-0 h-full w-full bg-cover bg-center bg-no-repeat"
                style={{
                  backgroundImage: `url(${project.image})`,
                }}
              />

              <div className="absolute inset-0 bg-black/35 transition-colors duration-500 group-hover:bg-black/45" />

              <div className="absolute inset-x-0 bottom-0 h-1/2 bg-linear-to-t from-black/60 via-black/10 to-transparent" />
              <Link
                href={project.link}
                target="_blank"
                rel="noopener noreferrer"
              >
                <div className="absolute bottom-3 left-3 right-3 flex items-center justify-between gap-4 rounded-xl border border-white/10 bg-black/50 px-5 py-4 backdrop-blur-xl transition-all duration-500 group-hover:border-white/20 group-hover:bg-black/60">
                  <div className="min-w-0">
                    <h3 className="truncate font-poppins text-sm font-medium text-white sm:text-base">
                      {project.title}
                    </h3>

                    <p className="mt-1 truncate font-poppins text-xs text-white/60">
                      {project.category}
                    </p>
                  </div>

                  <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-white/10 bg-white/5 transition-all duration-300 group-hover:border-white/20 group-hover:bg-white/10">
                    <svg
                      className="h-4 w-4 text-white transition-transform duration-300 group-hover:translate-x-0.5"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={1.5}
                        d="M5 12h14m-6-6 6 6-6 6"
                      />
                    </svg>
                  </div>
                </div>
              </Link>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

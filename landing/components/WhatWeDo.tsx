import { SERVICES } from "@/data/service";

export default function WhatWeDo() {
  return (
    <section
      id="service"
      className="relative overflow-hidden bg-black px-4 sm:px-6 lg:px-16 pt-10 sm:pt-12 md:pt-16 lg:pt-24"
    >
      <div
        className="pointer-events-none absolute inset-0 opacity-40"
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

      <div className="relative z-10 mx-auto max-w-7xl">
        <div className="max-w-4xl">
          <div className="mb-5 inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/[0.035] px-3.5 py-1.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)] backdrop-blur-xl sm:mb-7 sm:px-4">
            <span className="relative flex h-1.5 w-1.5">
              <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-white/30" />
              <span className="relative inline-flex h-1.5 w-1.5 rounded-full bg-white/70" />
            </span>

            <span className="font-poppins text-[9px] tracking-wide text-white/50 sm:text-xs">
              Our Services
            </span>
          </div>

          <h2 className="font-title text-[clamp(3rem,7vw,6rem)] font-medium leading-[0.9] tracking-[-0.055em] text-white">
            What We Do
          </h2>

          <p className="animate-fade-in-up mt-5 max-w-2xl font-poppins text-sm sm:text-base leading-relaxed text-light-gray [animation-delay:300ms] sm:mt-8">
            We design and build digital experiences, products and systems that
            turn ambitious ideas into something people can actually use.
          </p>
        </div>

        <div className="mt-10 grid grid-cols-1 gap-4 sm:mt-16 sm:grid-cols-2 sm:gap-3 lg:mt-20 lg:grid-cols-3">
          {SERVICES.map((card) => (
            <div
              key={card.number}
              className={`${card.className} group relative flex min-h-70 flex-col justify-between overflow-hidden rounded-xl border border-white/10 bg-white/[0.035] p-5 shadow-[inset_0_1px_0_rgba(255,255,255,0.07)] backdrop-blur-2xl backdrop-saturate-150 transition-all duration-500 ease-out hover:-translate-y-1 hover:border-white/18 hover:bg-white/5.5 hover:shadow-[0_20px_60px_rgba(0,0,0,0.45),inset_0_1px_0_rgba(255,255,255,0.1)] sm:min-h-75 sm:p-7 lg:min-h-85 lg:p-8`}
            >
              <div className="relative z-10">
                <div className="flex items-start justify-between">
                  <div className="flex h-10 w-10 items-center justify-center rounded-xl border border-white/[0.14] bg-white/6 text-white/70 shadow-[inset_0_1px_0_rgba(255,255,255,0.1)] backdrop-blur-xl transition-all duration-500 group-hover:border-white/22 group-hover:bg-white/10 group-hover:text-white group-hover:shadow-[0_0_25px_rgba(255,255,255,0.06)] sm:h-11 sm:w-11">
                    {card.icon}
                  </div>

                  <span className="font-poppins text-[10px] font-medium text-white/25 transition-colors duration-500 group-hover:text-white/50 sm:text-xs">
                    {card.number}
                  </span>
                </div>

                <h3 className="mt-6 font-title text-xl font-medium leading-tight tracking-tight text-white sm:mt-8 sm:text-2xl">
                  {card.title}
                </h3>

                <p className="mt-3 max-w-xl font-poppins text-[12px] sm:text-sm leading-relaxed text-white/40 transition-colors duration-500 group-hover:text-white/50 sm:mt-4">
                  {card.description}
                </p>
              </div>

              <div className="relative z-10 mt-8">
                <div className="flex flex-wrap gap-1.5 sm:gap-2">
                  {card.tags.map((tag) => (
                    <span
                      key={tag}
                      className="rounded-xl border border-white/10 bg-white/[0.035] px-2.5 py-1 font-poppins text-[10px] sm:text-[11px] text-white/50 shadow-[inset_0_1px_0_rgba(255,255,255,0.04)] backdrop-blur-md transition-all duration-300 group-hover:border-white/[0.14] group-hover:bg-white/5 group-hover:text-white/55 sm:px-3 sm:py-1.5"
                    >
                      {tag}
                    </span>
                  ))}
                </div>

                <div className="mt-5 h-px w-0 bg-linear-to-r from-white/30 via-white/10 to-transparent transition-all duration-700 group-hover:w-full" />
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

import { STATS } from "@/data/stat";

export default function WebResult() {
  return (
    <section className="relative overflow-hidden bg-black mt-10 sm:mt-12 md:mt-16 lg:mt-24">
      <div
        className="pointer-events-none absolute inset-0 opacity-70"
        style={{
          backgroundImage: `
            linear-gradient(rgba(255,255,255,0.025) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.025) 1px, transparent 1px)
          `,
          backgroundSize: "80px 80px",
          maskImage:
            "linear-gradient(to right, transparent, black 70%, black 90%, transparent)",
          WebkitMaskImage:
            "linear-gradient(to right, transparent, black 70%, black 90%, transparent)",
        }}
      />

      <div className="relative mx-auto max-w-7xl px-5 py-6 sm:px-8 md:px-12 lg:flex lg:min-h-175 lg:items-center lg:px-16 lg:py-0">
        <div className="relative z-20 w-full max-w-2xl lg:w-[52%]">
          <div className="animate-fade-in-up mb-6 inline-flex items-center gap-2.5 rounded-full border border-white/15 bg-white/5 px-4 py-2 shadow-[inset_0_1px_0_rgba(255,255,255,0.1)] backdrop-blur-xl transition-all duration-500 hover:border-white/25 hover:bg-white/8 sm:mb-8">
            <span className="relative flex h-2 w-2">
              <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-white opacity-50" />
              <span className="relative inline-flex h-2 w-2 rounded-full bg-white/80" />
            </span>

            <span className="font-poppins text-[10px] text-white/50 sm:text-xs">
              Our Approach
            </span>
          </div>

          <h2 className="animate-fade-in-up font-title text-[clamp(3rem,7vw,6rem)] font-medium leading-[0.9] tracking-[-0.055em] text-white [animation-delay:150ms] sm:text-6xl md:text-7xl lg:text-[5.5rem]">
            Web experiences
            <br />
            <span className="bg-linear-to-r from-white/30 to-white/10 bg-clip-text text-transparent">
              built to perform.
            </span>
          </h2>

          <p className="animate-fade-in-up mt-6 max-w-xl font-poppins text-sm sm:text-base leading-relaxed text-light-gray [animation-delay:300ms] sm:mt-8">
            From landing pages to complex web applications, we build fast,
            responsive and scalable digital experiences designed around real
            people.
          </p>
        </div>

        <div className="pointer-events-none absolute right-0 top-0 hidden h-full w-[48%] lg:block">
          {STATS.map((stat, index) => (
            <div
              key={stat.value}
              className={`animate-fade-in-up group absolute ${stat.position} ${stat.rotate} flex h-38.75 w-61.25 flex-col justify-between rounded-xl border border-white/12 bg-linear-to-br from-white/6 to-white/2 p-6 shadow-[inset_0_1px_0_rgba(255,255,255,0.15),0_8px_32px_rgba(0,0,0,0.3)] backdrop-blur-2xl backdrop-saturate-150 transition-all duration-700 hover:scale-105 hover:border-white/25 hover:from-white/10 hover:to-white/4 hover:shadow-[inset_0_1px_0_rgba(255,255,255,0.25),0_12px_40px_rgba(0,0,0,0.4)]`}
              style={{
                animationDelay: `${400 + index * 100}ms`,
              }}
            >
              <div className="relative z-10 flex justify-end">
                <div className="flex h-8 w-8 items-center justify-center rounded-lg border border-white/15 bg-white/8 text-white/60 shadow-[inset_0_1px_0_rgba(255,255,255,0.15)] backdrop-blur-xl transition-all duration-500 group-hover:scale-110 group-hover:border-white/30 group-hover:text-white">
                  <svg
                    className="h-3.5 w-3.5"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={1.5}
                      d="M7 17L17 7M17 7H8M17 7v9"
                    />
                  </svg>
                </div>
              </div>

              <div className="relative z-10">
                <div className="font-title text-4xl font-medium tracking-[-0.04em] text-white">
                  {stat.value}
                </div>

                <div className="mt-1 font-poppins text-[10px] uppercase tracking-[0.15em] text-white/30 transition-colors duration-500 group-hover:text-white/50">
                  {stat.label}
                </div>
              </div>
            </div>
          ))}
        </div>

        <div className="mt-10 grid grid-cols-2 gap-4 sm:mt-14 sm:gap-3 lg:hidden">
          {STATS.map((stat) => (
            <div
              key={stat.value}
              className="group relative min-h-33.75 overflow-hidden rounded-xl border border-white/12 bg-linear-to-br from-white/6 to-white/2 p-5 shadow-[inset_0_1px_0_rgba(255,255,255,0.15)] backdrop-blur-2xl transition-all duration-500 hover:border-white/25 hover:bg-white/8 sm:min-h-37.5 sm:p-6"
            >
              <div className="pointer-events-none absolute left-0 right-0 top-0 h-px bg-linear-to-r from-transparent via-white/25 to-transparent" />
              <div className="pointer-events-none absolute -right-10 -top-10 h-28 w-28 rounded-full bg-white/2.5 blur-3xl transition-all duration-500 group-hover:bg-white/5" />
              <div className="relative z-10 flex justify-end">
                <div className="flex h-7 w-7 items-center justify-center rounded-lg border border-white/12 bg-white/5 text-white/40 transition-all duration-500 group-hover:border-white/25 group-hover:text-white">
                  <svg
                    className="h-3 w-3"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={1.5}
                      d="M7 17L17 7M17 7H8M17 7v9"
                    />
                  </svg>
                </div>
              </div>
              <div className="relative z-10 mt-5 sm:mt-6">
                <div className="font-title text-3xl font-medium leading-none tracking-[-0.04em] text-white sm:text-4xl">
                  {stat.value}
                </div>

                <div className="mt-2 font-poppins text-[8px] uppercase tracking-[0.14em] text-white/30 sm:text-[10px] sm:tracking-[0.15em]">
                  {stat.label}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

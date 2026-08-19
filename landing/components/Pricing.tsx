"use client";

import { PRICES } from "@/data/price";

const Pricing = () => {
  return (
    <section
      id="service"
      className="relative w-full overflow-hidden bg-black px-4 sm:px-6 lg:px-16 pt-10 sm:pt-12 md:pt-16 lg:pt-24"
    >
      <div
        className="pointer-events-none absolute inset-0 opacity-50"
        style={{
          backgroundImage: `
            linear-gradient(rgba(255,255,255,0.025) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.025) 1px, transparent 1px)
          `,
          backgroundSize: "80px 80px",
          maskImage:
            "linear-gradient(to bottom, transparent, black 12%, black 88%, transparent)",
          WebkitMaskImage:
            "linear-gradient(to bottom, transparent, black 12%, black 88%, transparent)",
        }}
      />

      <div className="pointer-events-none absolute left-1/2 top-[42%] h-150 w-150 -translate-x-1/2 rounded-full bg-white/[0.018] blur-[160px]" />

      <div className="relative z-10 mx-auto max-w-7xl">
        <div className="mx-auto max-w-3xl text-center">
          <div className="mb-5 inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/[0.035] px-3.5 py-1.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)] backdrop-blur-xl sm:mb-7 sm:px-4">
            <span className="relative flex h-1.5 w-1.5">
              <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-white/30" />
              <span className="relative inline-flex h-1.5 w-1.5 rounded-full bg-white/70" />
            </span>

            <span className="font-poppins text-[9px] tracking-wide text-white/50 sm:text-xs">
              Pricing
            </span>
          </div>

          <h2 className="animate-fade-in-up font-title text-[clamp(3rem,8vw,5.5rem)] font-medium leading-[0.9] tracking-[-0.055em] text-white [animation-delay:150ms]">
            Flexible pricing
            <br />
            <span className="bg-linear-to-r from-white/35 via-white/20 to-white/10 bg-clip-text text-transparent">
              for every business.
            </span>
          </h2>

          <p className="animate-fade-in-up mx-auto mt-6 max-w-3xl font-poppins text-sm sm:text-base leading-relaxed text-light-gray [animation-delay:300ms] sm:mt-8">
            Straightforward packages for professionals and businesses. Choose
            what fits your needs today and scale when you're ready.
          </p>
        </div>

        <div className="mt-10 flex flex-col items-center gap-6 sm:mt-16 lg:mt-20 lg:grid lg:grid-cols-3 lg:items-center">
          {PRICES.map((service, index) => {
            const featured = service.isFeatured;

            return (
              <div
                key={service.title}
                className={`group relative w-full max-w-md lg:max-w-none ${featured ? "lg:-translate-y-3 lg:z-20" : "lg:z-10"} transition-transform duration-500 hover:-translate-y-1 ${featured ? "lg:hover:-translate-y-4" : ""}`}
              >
                {featured && (
                  <div className="absolute -top-3 left-1/2 z-30 -translate-x-1/2 whitespace-nowrap">
                    <div className="rounded-full border border-white/10 bg-white px-4 py-1.5 font-poppins text-[8px] uppercase tracking-[0.16em] font-semibold text-black shadow-[0_8px_30px_rgba(255,255,255,0.12)] sm:text-[9px]">
                      Most popular
                    </div>
                  </div>
                )}

                {featured && (
                  <div className="pointer-events-none absolute -inset-2 rounded-xl bg-white/2.5 opacity-0 blur-2xl transition-opacity duration-700 group-hover:opacity-100" />
                )}

                <div
                  className={`relative flex min-h-130 flex-col overflow-hidden rounded-xl p-6 sm:min-h-135 sm:p-8 ${
                    featured
                      ? "border border-white bg-white text-black shadow-[0_25px_90px_rgba(255,255,255,0.07)]"
                      : "border border-white/9 bg-linear-to-br from-white/5.5 via-white/3 to-white/1.5 text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.07),0_15px_50px_rgba(0,0,0,0.2)] backdrop-blur-2xl hover:border-white/17 hover:from-white/[0.07]"
                  } transition-all duration-500`}
                >
                  <div
                    className={`pointer-events-none absolute inset-x-8 top-0 h-px bg-linear-to-r from-transparent ${featured ? "via-black/20" : "via-white/25"} to-transparent`}
                  />

                  {!featured && (
                    <div className="pointer-events-none absolute -right-24 -top-24 h-48 w-48 rounded-xl bg-white/2.5 blur-3xl transition-all duration-700 group-hover:bg-white/5" />
                  )}

                  <div className="relative z-10">
                    <div className="flex items-center justify-between">
                      <span
                        className={`font-poppins text-xs font-medium sm:text-sm ${featured ? "text-black/60" : "text-white/60"}`}
                      >
                        {service.for}
                      </span>

                      <span
                        className={`font-poppins text-[13px] ${featured ? "text-black/60" : "text-white/60"}`}
                      >
                        0{index + 1}
                      </span>
                    </div>

                    <h3
                      className={`mt-7 font-title text-2xl font-medium leading-tight tracking-tight sm:mt-8 sm:text-3xl ${featured ? "text-black" : "text-white"}`}
                    >
                      {service.title}
                    </h3>

                    <div className="mt-8">
                      <div>
                        <p
                          className={`mb-2 font-poppins text-[10px] uppercase tracking-[0.12em] ${
                            featured ? "text-black/60" : "text-white/60"
                          }`}
                        >
                          {service.priceLabel}
                        </p>

                        <div className="flex items-baseline gap-2">
                          <span
                            className={`font-title text-[3.2rem] font-medium leading-none tracking-[-0.06em] sm:text-[4rem] ${
                              featured ? "text-black" : "text-white"
                            }`}
                          >
                            {service.price.toLocaleString()}
                          </span>

                          <span
                            className={`font-poppins text-xs ${
                              featured ? "text-black/35" : "text-white/30"
                            }`}
                          >
                            ETB
                          </span>
                        </div>
                      </div>
                    </div>

                    <p
                      className={`mt-6 max-w-sm font-poppins text-[11px] leading-relaxed sm:text-xs ${featured ? "text-black/60" : "text-white/60"}`}
                    >
                      {service.shortDescription}
                    </p>

                    {/* Best for tags */}
                    <div className="mt-4 flex flex-wrap gap-1.5">
                      {service.bestFor.map((item) => (
                        <span
                          key={item}
                          className={`rounded-lg px-2 py-1 font-poppins text-[9px] ${
                            featured
                              ? "bg-black/5 text-black/60"
                              : "bg-white/5 text-white/50"
                          }`}
                        >
                          {item}
                        </span>
                      ))}
                    </div>
                  </div>

                  <div className="relative z-10 mt-7 flex-1 sm:mt-8">
                    <div
                      className={`mb-5 h-px w-full ${featured ? "bg-black/10" : "bg-white/[0.07]"}`}
                    />

                    {/* Quick info */}
                    <div className="mb-5 grid grid-cols-2 gap-3">
                      <div>
                        <p
                          className={`font-poppins text-[9px] uppercase tracking-wide ${featured ? "text-black/40" : "text-white/40"}`}
                        >
                          Pages
                        </p>
                        <p
                          className={`mt-1 font-poppins text-[11px] font-medium ${featured ? "text-black/70" : "text-white/70"}`}
                        >
                          {service.pagesIncluded}
                        </p>
                      </div>
                      <div>
                        <p
                          className={`font-poppins text-[9px] uppercase tracking-wide ${featured ? "text-black/40" : "text-white/40"}`}
                        >
                          Delivery
                        </p>
                        <p
                          className={`mt-1 font-poppins text-[11px] font-medium ${featured ? "text-black/70" : "text-white/70"}`}
                        >
                          {service.delivery}
                        </p>
                      </div>
                      <div>
                        <p
                          className={`font-poppins text-[9px] uppercase tracking-wide ${featured ? "text-black/40" : "text-white/40"}`}
                        >
                          Revisions
                        </p>
                        <p
                          className={`mt-1 font-poppins text-[11px] font-medium ${featured ? "text-black/70" : "text-white/70"}`}
                        >
                          {service.revisions}
                        </p>
                      </div>
                      <div>
                        <p
                          className={`font-poppins text-[9px] uppercase tracking-wide ${featured ? "text-black/40" : "text-white/40"}`}
                        >
                          Support
                        </p>
                        <p
                          className={`mt-1 font-poppins text-[11px] font-medium ${featured ? "text-black/70" : "text-white/70"}`}
                        >
                          {service.supportPeriod}
                        </p>
                      </div>
                    </div>

                    <div
                      className={`mb-5 h-px w-full ${featured ? "bg-black/10" : "bg-white/[0.07]"}`}
                    />

                    <div className="space-y-3.5">
                      {service.description.slice(0, 5).map((item) => (
                        <div key={item} className="flex items-start gap-3">
                          {/* Check */}
                          <span
                            className={`mt-0.75 flex h-4 w-4 shrink-0 items-center justify-center rounded-xl ${
                              featured
                                ? "bg-black"
                                : "border border-white/12 bg-white/6"
                            }`}
                          >
                            <svg
                              className={`h-2.5 w-2.5 ${featured ? "text-white" : "text-white/60"}`}
                              fill="none"
                              viewBox="0 0 24 24"
                              stroke="currentColor"
                            >
                              <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth={3}
                                d="M5 13l4 4L19 7"
                              />
                            </svg>
                          </span>

                          <span
                            className={`font-poppins text-[11px] leading-4 ${featured ? "text-black/60" : "text-white/60"}`}
                          >
                            {item}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>

                  <button
                    className={`group/button relative z-10 mt-8 flex w-full cursor-pointer items-center justify-center gap-2 overflow-hidden rounded-xl px-5 py-3.5 font-poppins text-xs font-medium transition-all duration-300 ${
                      featured
                        ? "bg-black text-white hover:bg-black/80"
                        : "border border-white/10 bg-white/4.5 text-white/65 backdrop-blur-xl hover:border-white/20 hover:bg-white/9 hover:text-white"
                    }`}
                  >
                    <span>Get started</span>

                    <svg
                      className="h-3.5 w-3.5 transition-transform duration-300 group-hover/button:translate-x-1"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={1.8}
                        d="M5 12h14m-6-6l6 6-6 6"
                      />
                    </svg>
                  </button>
                </div>
              </div>
            );
          })}
        </div>

        <div className="mt-10 flex items-center justify-center gap-3 sm:mt-12">
          <p className="font-poppins text-[11px] text-white/60 sm:text-[12px]">
            Need something different?{" "}
            <button className="text-white/50 underline underline-offset-4 cursor-pointer transition-colors hover:text-white">
              Let's talk about your project.
            </button>
          </p>
        </div>
      </div>
    </section>
  );
};

export default Pricing;

"use client";

import { useState } from "react";
import { FAQS } from "@/data/faq";

export default function FAQ() {
  const [openIndex, setOpenIndex] = useState(1);

  return (
    <section className="relative overflow-hidden bg-black px-4 sm:px-6 lg:px-16 pt-10 sm:pt-12 md:pt-16 lg:pt-24">
      {/* Subtle grid */}
      <div
        className="pointer-events-none absolute inset-0 opacity-25"
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

      <div className="relative mx-auto max-w-9xl">
        <div className="grid grid-cols-1 gap-16 md:grid-cols-[0.8fr_1.2fr] md:items-start md:gap-16 lg:gap-24">
          <div>
            <div className="mb-5 inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/[0.035] px-3.5 py-1.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)] backdrop-blur-xl sm:mb-7 sm:px-4">
              <span className="relative flex h-1.5 w-1.5">
                <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-white/30" />
                <span className="relative inline-flex h-1.5 w-1.5 rounded-full bg-white/70" />
              </span>

              <span className="font-poppins text-[9px] tracking-wide text-white/50 sm:text-xs">
                FAQ
              </span>
            </div>

            <h2 className="font-title text-[clamp(3rem,7vw,6rem)] font-medium leading-[0.9] tracking-[-0.055em] text-white">
              Your questions
              <br />
              <span className="text-white/30">answered.</span>
            </h2>

            <p className="animate-fade-in-up mt-5 max-w-2xl font-poppins text-[13px] sm:text-[14px] leading-relaxed text-light-gray [animation-delay:300ms] sm:mt-8">
              Everything you need to know about working with Atronsa. Can't find
              what you're looking for?
            </p>
          </div>

          <div className="space-y-2">
            {FAQS.map((faq, index) => {
              const isOpen = openIndex === index;

              return (
                <div
                  key={faq.question}
                  className={`group relative overflow-hidden rounded-xl border transition-all duration-500 cursor-pointer ${
                    isOpen
                      ? "border-white/20 bg-white/92 text-black"
                      : "border-white/[0.07] bg-white/[0.035] text-white hover:border-white/13 hover:bg-white/5"
                  }`}
                >
                  {!isOpen && (
                    <div className="pointer-events-none absolute inset-x-6 top-0 h-px bg-linear-to-r from-transparent via-white/20 to-transparent" />
                  )}

                  <button
                    onClick={() => setOpenIndex(isOpen ? -1 : index)}
                    className="relative z-10 flex w-full cursor-pointer items-center justify-between gap-6 px-5 py-5 text-left sm:px-6"
                  >
                    <span
                      className={`font-poppins text-sm font-medium transition-colors ${
                        isOpen
                          ? "text-black"
                          : "text-white/65 group-hover:text-white/90"
                      }`}
                    >
                      {faq.question}
                    </span>

                    <span
                      className={`flex h-6 w-6 shrink-0 items-center justify-center rounded-full border transition-all duration-300 ${
                        isOpen
                          ? "border-black/10 bg-black/5"
                          : "border-white/10 bg-white/4"
                      }`}
                    >
                      <svg
                        className={`h-3.5 w-3.5 transition-transform duration-300 ${
                          isOpen ? "rotate-45" : ""
                        }`}
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={1.5}
                          d="M12 5v14M5 12h14"
                        />
                      </svg>
                    </span>
                  </button>

                  {/* Answer */}
                  <div
                    className={`grid transition-[grid-template-rows,opacity] duration-500 ease-out ${
                      isOpen
                        ? "grid-rows-[1fr] opacity-100"
                        : "grid-rows-[0fr] opacity-0"
                    }`}
                  >
                    <div className="overflow-hidden">
                      <div className="px-5 pb-6 sm:px-6">
                        <div
                          className={`mb-5 h-px ${
                            isOpen ? "bg-black/10" : "bg-white/10"
                          }`}
                        />

                        <p
                          className={`max-w-2xl font-poppins text-[12px] sm:text-xs leading-6 ${
                            isOpen ? "text-black/90" : "text-white/90"
                          }`}
                        >
                          {faq.answer}
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </section>
  );
}

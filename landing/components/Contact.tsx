"use client";

import { useState, useEffect } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { contactSchema, type ContactFormValues } from "@/schema/contact";
import { CONTACT_INFO } from "@/data/contact";
import PhoneInput from "@/components/ui/PhoneInput";
import ProjectTypeSelect from "./ui/ProjectTypeSelect";

export default function Contact() {
  const [submitted, setSubmitted] = useState(false);
  const [submitError, setSubmitError] = useState<string | null>(null);
  const [phoneCode, setPhoneCode] = useState("+251");
  const [phoneNumber, setPhoneNumber] = useState("");

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
    setValue,
    trigger,
    watch,
  } = useForm<ContactFormValues>({
    resolver: zodResolver(contactSchema),
    defaultValues: {
      name: "",
      email: "",
      phone: "",
      projectType: undefined,
      message: "",
      website: "",
    },
  });

  useEffect(() => {
    const handlePrefill = (e: Event) => {
      const detail = (e as CustomEvent<{ projectType: string }>).detail;
      if (detail?.projectType) {
        setValue(
          "projectType",
          detail.projectType as ContactFormValues["projectType"],
          {
            shouldValidate: true,
          },
        );
      }
    };

    window.addEventListener("prefill-contact", handlePrefill);
    return () => window.removeEventListener("prefill-contact", handlePrefill);
  }, [setValue]);

  const handlePhoneChange = (value: string) => {
    setPhoneNumber(value);
    const fullPhone = `${phoneCode}${value.replace(/\s/g, "")}`;
    setValue("phone", fullPhone);
  };

  const handlePhoneCodeChange = (code: string) => {
    setPhoneCode(code);
    const fullPhone = `${code}${phoneNumber.replace(/\s/g, "")}`;
    setValue("phone", fullPhone);
  };

  const onSubmit = async (data: ContactFormValues) => {
    setSubmitError(null);

    try {
      const res = await fetch("/api/contact", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          name: data.name,
          email: data.email,
          phone: data.phone,
          projectType: data.projectType,
          message: data.message,
          website: data.website || "",
        }),
      });

      const result = await res.json();

      if (!res.ok) {
        if (result.details) {
          const details = result.details as Record<string, string[]>;
          const firstError = Object.values(details)[0]?.[0];
          throw new Error(firstError || "Validation failed");
        }
        throw new Error(
          result.error || "Something went wrong. Please try again.",
        );
      }

      setSubmitted(true);
      reset();
      setPhoneNumber("");
      setPhoneCode("+251");
    } catch (err) {
      setSubmitError(
        err instanceof Error
          ? err.message
          : "Something went wrong. Please try again.",
      );
    }
  };

  return (
    <section
      id="contact"
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

      <div className="relative mx-auto max-w-7xl">
        <div className="text-center max-w-3xl mx-auto">
          <div className="mb-5 inline-flex items-center gap-2 rounded-full border border-white/10 bg-white/[0.035] px-3.5 py-1.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.06)] backdrop-blur-xl sm:mb-7 sm:px-4">
            <span className="relative flex h-1.5 w-1.5">
              <span className="absolute inline-flex h-full w-full animate-ping rounded-full bg-white/30" />
              <span className="relative inline-flex h-1.5 w-1.5 rounded-full bg-white/70" />
            </span>

            <span className="font-poppins text-[9px] tracking-wide text-white/50 sm:text-xs">
              Contact Our Web Agency in Addis Ababa
            </span>
          </div>

          <h2 className="font-title text-[clamp(3rem,7vw,6rem)] font-medium leading-[0.9] tracking-[-0.055em] text-white">
            Let's build
            <br />
            <span className="text-white/30">something.</span>
          </h2>

          <p className="animate-fade-in-up mx-auto mt-6 max-w-3xl font-poppins text-sm sm:text-base leading-relaxed text-light-gray [animation-delay:300ms] sm:mt-8">
            Have an idea, a project, or a website that needs a serious upgrade?
            Tell us what you're working on and we'll get back to you.
          </p>
        </div>

        <div className="mt-10 grid grid-cols-1 gap-5 md:grid-cols-[0.75fr_1.25fr]">
          <div className="flex flex-col justify-between rounded-xl border border-white/8 bg-white/2.5 p-7 backdrop-blur-xl sm:p-9">
            <div>
              <p className="font-poppins text-[10px] uppercase tracking-[0.12em] text-white/35">
                Contact
              </p>

              <p className="mt-4 max-w-full md:max-w-sm font-poppins text-[11px] sm:text-xs leading-relaxed text-light-gray">
                Looking for a web agency in Addis Ababa? Prefer to reach us
                directly? You can contact us through email, phone, or social
                media for all your web development needs.
              </p>
            </div>

            <div className="mt-10 space-y-3">
              {CONTACT_INFO.methods.map((method) => (
                <a
                  key={method.label}
                  href={method.href}
                  target={method.external ? "_blank" : undefined}
                  rel={method.external ? "noopener noreferrer" : undefined}
                  className="group block rounded-xl border border-white/[0.07] bg-white/2.5 p-4 transition-all duration-300 hover:border-white/15 hover:bg-white/5"
                >
                  <div className="flex items-center justify-between">
                    <div>
                      <p className="font-poppins text-[10px] uppercase tracking-[0.12em] text-white/35">
                        {method.label}
                      </p>

                      <p className="mt-1.5 font-poppins text-xs sm:text-sm text-white/70 transition-colors group-hover:text-white">
                        {method.value}
                      </p>
                    </div>

                    <div className="flex h-9 w-9 items-center justify-center rounded-full border border-white/10 bg-white/4">
                      {method.icon}
                    </div>
                  </div>
                </a>
              ))}
            </div>

            <div className="mt-8 border-t border-white/[0.07] pt-5">
              <p className="mb-3 font-poppins text-[10px] uppercase tracking-[0.12em] text-white/35">
                Follow us
              </p>

              <div className="flex items-center gap-2.5">
                {CONTACT_INFO.socials.map((social) => (
                  <a
                    key={social.name}
                    href={social.href}
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label={social.name}
                    className="flex h-9 w-9 items-center justify-center rounded-full border border-white/10 bg-white/4 text-white/50 transition-all duration-300 hover:border-white/25 hover:bg-white/10 hover:text-white"
                  >
                    {social.icon}
                  </a>
                ))}
              </div>
            </div>

            <div className="mt-8 border-t border-white/[0.07] pt-5">
              <div className="flex items-center gap-2">
                <span className="h-1.5 w-1.5 rounded-full bg-emerald-400/70" />

                <span className="font-poppins text-[11px] sm:text-xs text-white/30">
                  Usually responds within 24 hours
                </span>
              </div>
            </div>
          </div>

          <div className="relative overflow-hidden rounded-xl border border-white/9 bg-white/[0.035] p-7 backdrop-blur-2xl sm:p-9">
            <div className="pointer-events-none absolute inset-x-10 top-0 h-px bg-linear-to-r from-transparent via-white/25 to-transparent" />
            {submitted ? (
              <div className="flex min-h-130 flex-col items-center justify-center text-center">
                <div className="flex h-12 w-12 items-center justify-center rounded-full border border-white/10 bg-white/6">
                  <svg
                    className="h-6 w-6 text-white"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={1.5}
                      d="M5 13l4 4L19 7"
                    />
                  </svg>
                </div>

                <h3 className="mt-6 font-title text-3xl text-white">
                  Message sent.
                </h3>

                <p className="mt-3 max-w-sm font-poppins text-sm leading-6 text-white/40">
                  Thanks for reaching out. We'll get back to you as soon as
                  possible.
                </p>

                <button
                  onClick={() => setSubmitted(false)}
                  className="mt-7 font-poppins cursor-pointer text-[11px] sm:text-xs text-white/50 underline underline-offset-4 transition-colors hover:text-white"
                >
                  Send another message
                </button>
              </div>
            ) : (
              <form
                onSubmit={handleSubmit(onSubmit)}
                className="relative z-10"
                noValidate
                autoComplete="off"
              >
                <div className="mb-9">
                  <p className="font-poppins text-[10px] uppercase tracking-[0.12em] text-white/35">
                    Start a conversation
                  </p>

                  <h3 className="mt-3 font-title text-2xl text-white sm:text-3xl">
                    Tell us about your project.
                  </h3>
                </div>

                <input
                  type="text"
                  className="hidden"
                  {...register("website")}
                  tabIndex={-1}
                  autoComplete="off"
                />

                <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
                  <div>
                    <label
                      htmlFor="name"
                      className="mb-2 block font-poppins text-[11px] sm:text-[13px] text-light-gray"
                    >
                      Full name
                    </label>

                    <input
                      id="name"
                      type="text"
                      placeholder="Your full name"
                      disabled={isSubmitting}
                      {...register("name")}
                      className={`w-full border-b bg-transparent py-2 font-poppins text-[11px] sm:text-[13px] text-light-gray outline-none placeholder:text-white/20 transition-colors focus:border-white/50 disabled:cursor-not-allowed disabled:opacity-50 ${
                        errors.name ? "border-red-400/60" : "border-white/12"
                      }`}
                    />
                    {errors.name && (
                      <p className="mt-1.5 font-poppins text-[11px] text-red-400/80">
                        {errors.name.message}
                      </p>
                    )}
                  </div>

                  <div>
                    <label
                      htmlFor="email"
                      className="mb-2 block font-poppins text-[11px] sm:text-[13px] text-light-gray"
                    >
                      Email address
                    </label>

                    <input
                      id="email"
                      type="email"
                      placeholder="you@example.com"
                      disabled={isSubmitting}
                      {...register("email")}
                      className={`w-full border-b bg-transparent py-2 font-poppins text-[11px] sm:text-[13px] text-light-gray outline-none placeholder:text-white/20 transition-colors focus:border-white/50 disabled:cursor-not-allowed disabled:opacity-50 ${
                        errors.email ? "border-red-400/60" : "border-white/12"
                      }`}
                    />
                    {errors.email && (
                      <p className="mt-1.5 font-poppins text-[11px] text-red-400/80">
                        {errors.email.message}
                      </p>
                    )}
                  </div>
                </div>

                <div className="mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2">
                  <div>
                    <label
                      htmlFor="phone"
                      className="mb-2 block font-poppins text-[11px] sm:text-[13px] text-light-gray"
                    >
                      Phone number
                    </label>
                    <PhoneInput
                      phoneCode={phoneCode}
                      setPhoneCode={handlePhoneCodeChange}
                      phoneNumber={phoneNumber}
                      setPhoneNumber={handlePhoneChange}
                      onBlur={() => trigger("phone")}
                      error={errors.phone?.message}
                      disabled={isSubmitting}
                    />
                    <input type="hidden" {...register("phone")} />
                  </div>

                  <div>
                    <label
                      htmlFor="projectType"
                      className="mb-2 block font-poppins text-[11px] sm:text-[13px] text-light-gray"
                    >
                      Project type
                    </label>

                    <ProjectTypeSelect
                      value={watch("projectType")}
                      onChange={(value) =>
                        setValue("projectType", value, { shouldValidate: true })
                      }
                      onBlur={() => trigger("projectType")}
                      error={errors.projectType?.message}
                      disabled={isSubmitting}
                    />
                  </div>
                </div>

                <div className="mt-7">
                  <label
                    htmlFor="message"
                    className="mb-2 block font-poppins text-[11px] sm:text-[13px] text-light-gray"
                  >
                    Message
                  </label>

                  <textarea
                    id="message"
                    rows={5}
                    placeholder="Tell us what you're building..."
                    disabled={isSubmitting}
                    {...register("message")}
                    className={`w-full resize-none border-b bg-transparent font-poppins py-2 leading-6 text-[11px] sm:text-[13px] text-light-gray outline-none placeholder:text-white/20 transition-colors focus:border-white/50 disabled:cursor-not-allowed disabled:opacity-50 ${
                      errors.message ? "border-red-400/60" : "border-white/12"
                    }`}
                  />
                  {errors.message && (
                    <p className="mt-1.5 font-poppins text-[11px] text-red-400/80">
                      {errors.message.message}
                    </p>
                  )}
                </div>

                {submitError && (
                  <p className="mt-4 font-poppins text-[11px] text-red-400/80">
                    {submitError}
                  </p>
                )}

                <button
                  type="submit"
                  disabled={isSubmitting}
                  className="group relative mt-9 flex w-full items-center justify-center gap-3 overflow-hidden rounded-xl cursor-pointer border border-white/20 bg-white/8 px-6 py-4 font-poppins text-[11px] sm:text-[13px] text-light-gray backdrop-blur-xl transition-all duration-300 hover:border-white/30 hover:bg-white/13 disabled:cursor-not-allowed disabled:opacity-50"
                >
                  {isSubmitting ? (
                    <>
                      <svg
                        className="h-4 w-4 animate-spin text-light-gray"
                        fill="none"
                        viewBox="0 0 24 24"
                      >
                        <circle
                          className="opacity-25"
                          cx="12"
                          cy="12"
                          r="10"
                          stroke="currentColor"
                          strokeWidth="4"
                        />
                        <path
                          className="opacity-75"
                          fill="currentColor"
                          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                        />
                      </svg>
                      <span>Sending...</span>
                    </>
                  ) : (
                    <>
                      <span>Send message</span>
                      <svg
                        className="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1"
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
                    </>
                  )}

                  {!isSubmitting && (
                    <span className="pointer-events-none absolute inset-y-0 left-full w-1/2 skew-x-[-20deg] bg-linear-to-r from-transparent via-white/10 to-transparent transition-all duration-700 group-hover:left-[120%]" />
                  )}
                </button>

                <p className="mt-4 text-center font-poppins text-[11px] sm:text-xs text-white/30">
                  By submitting this form, you agree to be contacted regarding
                  your inquiry.
                </p>
              </form>
            )}
          </div>
        </div>
      </div>
    </section>
  );
}

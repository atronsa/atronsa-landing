"use client";

import { Suspense, useState, type FormEvent } from "react";
import { signIn } from "next-auth/react";
import { useRouter, useSearchParams } from "next/navigation";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { contactSchema } from "@/schema/contact";

function EyeIcon({ className = "h-4 w-4" }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      strokeWidth={1.5}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z"
      />
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
      />
    </svg>
  );
}

function EyeOffIcon({ className = "h-4 w-4" }: { className?: string }) {
  return (
    <svg
      className={className}
      fill="none"
      viewBox="0 0 24 24"
      stroke="currentColor"
      strokeWidth={1.5}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.773 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.242 4.242L9.88 9.88"
      />
    </svg>
  );
}

const loginSchema = z.object({
  email: contactSchema.shape.email,
  password: z.string().min(1, "Enter your password"),
});

type LoginFormValues = z.infer<typeof loginSchema>;

function LoginForm() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const callbackUrl = searchParams.get("callbackUrl") || "/admin";

  const [showPassword, setShowPassword] = useState(false);
  const [submitError, setSubmitError] = useState<string | null>(null);

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
    defaultValues: { email: "", password: "" },
  });

  const onSubmit = async (data: LoginFormValues) => {
    setSubmitError(null);

    const res = await signIn("credentials", {
      email: data.email,
      password: data.password,
      redirect: false,
    });

    if (res?.error) {
      setSubmitError("Invalid email or password");
      return;
    }

    router.push(callbackUrl);
    router.refresh();
  };

  return (
    <main className="flex min-h-screen items-center justify-center bg-black px-4">
      <div className="w-full max-w-sm rounded-xl border border-white/9 bg-white/[0.035] p-8 backdrop-blur-2xl">
        <h1 className="mt-2 font-title text-center text-2xl text-white">
          Sign In
        </h1>

        <form
          onSubmit={handleSubmit(onSubmit)}
          autoComplete="off"
          noValidate
          className={`mt-8 ${isSubmitting ? "cursor-not-allowed" : ""}`}
        >
          <div className="mb-5">
            <label
              htmlFor="email"
              className="mb-2 block font-poppins text-[11px] text-white/70"
            >
              Email
            </label>
            <input
              id="email"
              type="email"
              disabled={isSubmitting}
              {...register("email")}
              className="w-full border-b border-white/12 bg-transparent py-2 font-poppins text-[11px] sm:text-[13px] text-light-gray outline-none placeholder:text-white/20 transition-colors focus:border-white/50 disabled:cursor-not-allowed disabled:opacity-50"
            />
            {errors.email && (
              <p className="mt-1.5 font-poppins text-[11px] text-red-400/80">
                {errors.email.message}
              </p>
            )}
          </div>

          <div>
            <label
              htmlFor="password"
              className="mb-2 block font-poppins text-[11px] text-white/70"
            >
              Password
            </label>

            <div className="relative">
              <input
                id="password"
                type={showPassword ? "text" : "password"}
                disabled={isSubmitting}
                {...register("password")}
                className="w-full border-b border-white/12 bg-transparent py-2 pr-8 font-poppins text-[11px] sm:text-[13px] text-light-gray outline-none placeholder:text-white/20 transition-colors focus:border-white/50 disabled:cursor-not-allowed disabled:opacity-50"
              />

              <button
                type="button"
                onClick={() => setShowPassword((prev) => !prev)}
                disabled={isSubmitting}
                tabIndex={-1}
                aria-label={showPassword ? "Hide password" : "Show password"}
                className="absolute cursor-pointer right-0 top-1/2 -translate-y-1/2 text-white/70 transition-colors hover:text-white disabled:cursor-not-allowed disabled:opacity-50"
              >
                {showPassword ? (
                  <EyeOffIcon className="h-4 w-4" />
                ) : (
                  <EyeIcon className="h-4 w-4" />
                )}
              </button>
            </div>

            {errors.password && (
              <p className="mt-1.5 font-poppins text-[11px] text-red-400/80">
                {errors.password.message}
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
            className="mt-8 w-full cursor-pointer rounded-xl border border-white/20 bg-white/8 py-3 font-poppins text-[11px] sm:text-[13px] text-light-gray transition-all duration-300 hover:border-white/30 hover:bg-white/[0.13] disabled:cursor-not-allowed disabled:opacity-50"
          >
            {isSubmitting ? "Signing In..." : "Sign In"}
          </button>
        </form>
      </div>
    </main>
  );
}

export default function LoginPage() {
  return (
    <Suspense
      fallback={
        <div className="flex min-h-screen items-center justify-center bg-black">
          <p className="text-white/70">Loading...</p>
        </div>
      }
    >
      <LoginForm />
    </Suspense>
  );
}

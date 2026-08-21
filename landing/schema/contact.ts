import { z } from "zod";
import { isValidPhoneNumber } from "libphonenumber-js";

export const contactSchema = z.object({
  name: z
    .string()
    .trim()
    .min(2, "Enter your full name")
    .max(80, "Name is too long")
    .refine((val) => val.trim().split(/\s+/).length >= 2, {
      message: "Please enter your first and last name",
    }),

  email: z
    .string()
    .trim()
    .min(1, "Enter your email")
    .max(254, "Email is too long")
    .email("Enter a valid email address"),

  phone: z
    .string()
    .min(1, "Enter your phone number")
    .refine((val) => isValidPhoneNumber(val), {
      message: "Enter a valid phone number",
    }),

  projectType: z.enum(["personal", "business", "revamp", "other"], {
    message: "Select a project type",
  }),

  message: z
    .string()
    .trim()
    .min(10, "Message must be at least 10 characters")
    .max(500, "Message must be less than 500 characters"),

  website: z.string().max(0, "Spam detected").optional(),
});

export type ContactFormValues = z.infer<typeof contactSchema>;

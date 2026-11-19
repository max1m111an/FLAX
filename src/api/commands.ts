import { FaCommands } from "@/api/faCommands.ts";
import { MealyCommands } from "@/api/mealyCommands.ts";

export interface ApiCommands extends FaCommands, MealyCommands {}
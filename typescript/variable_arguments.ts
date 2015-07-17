function variable_arguments(...args: number[]): number[] {
  return args;
}

variable_arguments();
variable_arguments(0);
variable_arguments(0, 1);

function variable_arguments_with_template<T>(...args: T[]): T[] {
  return args;
}

variable_arguments_with_template<number>(1, 2, 3);
variable_arguments_with_template<string>("a");
variable_arguments_with_template<boolean[]>([true, false]);

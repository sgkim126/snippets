function overloaded(n1: number): boolean;
function overloaded(s: string): number;
function overloaded(a: any): any {
  if (typeof a === 'number') {
    return a === 0;
  }
  if (typeof a === 'string') {
    return a.length;
  }

  throw new Error('not matched type');
}

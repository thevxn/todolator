declare global {
  declare const __brand: unique symbol
  type Branded<T, Brand> = T & { [__brand]: Brand }
}

export {}

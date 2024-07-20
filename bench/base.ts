export const iterations = process.env.BENCH_COUNT ? parseInt(process.env.BENCH_COUNT, 10) : 20

export const TEST_URL = 'https://choosealicense.com'
export const TEST_URL_MEDIUM = 'https://rsseau.fr'
export const TEST_URL_LARGE = 'https://espn.com'

export enum BenchSizes {
  SMALL = 'SMALL',
  MEDIUM = 'MEDIUM',
  LARGE = 'LARGE',
}

export const iterations = process.env.BENCH_COUNT
  ? parseInt(process.env.BENCH_COUNT, 10)
  : 20;

export const TEST_URL = "https://choosealicense.com";

export const parseErrorMessage = (error: any): string => {
  if (typeof error !== "string") return "알수 없는 에러입니다. (target is not string)";
  let data = error.split("::");
  if (data.length < 2) return "알수 없는 에러입니다. (split error)";
  return data[1];
}

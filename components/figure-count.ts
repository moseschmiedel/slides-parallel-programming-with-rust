import { ref } from "vue";

const totalFigureCount = ref(0);

export function useFigureNumber() {
  totalFigureCount.value++;
  const figureNumber = ref(totalFigureCount.value);
  return figureNumber;
}

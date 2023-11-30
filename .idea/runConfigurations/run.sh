for D in {2..25};
do
  cat "day_1___run.xml" | sed "s/day 1/day ${D}/g" | sed "s/day-1/day-${D}/g" > "day_${D}___run.xml"
  cat "day_1___tests.xml" | sed "s/day 1/day ${D}/g" | sed "s/day-1/day-${D}/g" > "day_${D}___tests.xml"
done


"use client";
import { useEffect, useState } from "react";
import { driver, Side } from "driver.js";
import { getUniqueSelector } from "./utils";
import "driver.js/dist/driver.css";

const driverObj = driver();

export const TourDriver = () => {
  const [isTourEnded, setIsTourEnded] = useState(false);

  useEffect(() => {
    const elements = Array.from(document.querySelectorAll("[data-tour]"));
    const steps = elements.map((element) => ({
      element: getUniqueSelector(element),
      popover: {
        title: element.getAttribute("data-tour") ?? "",
        description: element.getAttribute("data-tour-description") ?? "",
        side: element.getAttribute("data-tour-position") as Side,
      },
    }));
    driverObj.setConfig({
      steps,
      popoverClass: "tour-wrapper",
      onDestroyStarted: () => {
        if (!driverObj.hasNextStep()) {
          driverObj.destroy();
          setIsTourEnded(true);
        }
      },
    });
    driverObj.drive();
  }, []);

  return isTourEnded ? null : <></>;
};

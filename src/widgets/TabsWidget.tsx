import "/src/assets/scss/index.scss";
import { ROUTES } from "@/configs/RoutesConst.ts";
import { NavLink } from "react-router-dom";
import HomeIcon from "@/assets/svg/HomeIcon.svg?react";

export default function TabsWidget() {
    return (
        <div className="tabs_wrapper">
            <NavLink to={ ROUTES.MAIN } className="tab">
                <HomeIcon />
                Главная
            </NavLink>
            <NavLink to={ ROUTES.SETTINGS } className="tab">
                Настройки
            </NavLink>
        </div>
    );
}

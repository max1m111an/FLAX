import "@/assets/scss/index.scss";
import { ROUTES } from "@/configs/RoutesConst.ts";
import { NavLink } from "react-router-dom";
import HomeIcon from "@/assets/svg/Home.svg?react";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import Plus from "@/assets/svg/Plus.svg?react";
import Cancel from "@/assets/svg/Cancel.svg?react";
import { models } from "@/data/models.ts";
import { useTabs } from "@/context/TabsContext.tsx";


export default function TabsWidget() {
    const { tabs, addTab, removeTab } = useTabs();


    return (
        <div className="tabs_wrapper">
            <NavLink to={ ROUTES.MAIN } className="tab">
                <HomeIcon />
                Главная
            </NavLink>
            {tabs.map((tab) => {
                return (
                    <>
                        <NavLink to={ `/models/${tab.id}` } className="tab">
                            <tab.model.icon />
                            {tab.title}
                            <Cancel
                                className="cancel-icon"
                                onClick={ (e) => {
                                    e.preventDefault();
                                    e.stopPropagation();
                                    removeTab(tab);
                                } }
                            />
                        </NavLink>
                    </>
                );
            })}

            <DropdownMenu.Root>
                <DropdownMenu.Trigger className="plus-button">
                    <Plus />
                </DropdownMenu.Trigger>

                <DropdownMenu.Content className="dropdown">
                    <DropdownMenu.Group>
                        {models.map((model) => {
                            return (
                                <>
                                    <DropdownMenu.Item className="item" onClick={ () => addTab(model) }>
                                        <model.icon />
                                        {model.type}
                                    </DropdownMenu.Item>
                                </>
                            );
                        })}
                    </DropdownMenu.Group>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </div>
    );
}

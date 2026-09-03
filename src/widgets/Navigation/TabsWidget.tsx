import { ROUTES } from "@/configs/RoutesConst.ts";
import { NavLink } from "react-router-dom";
import HomeIcon from "@/assets/svg/Home.svg?react";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import Plus from "@/assets/svg/Plus.svg?react";
import Cancel from "@/assets/svg/Cancel.svg?react";
import { models } from "@/data/models.ts";
import { useTabs } from "@/context/TabsContext.tsx";
import clsx from "clsx";
import styles from "./TabsWidget.module.scss";


export default function TabsWidget() {
    const { tabs, addTab, removeTab } = useTabs();


    return (
        <div className={ styles.tabsWrapper }>
            <NavLink to={ ROUTES.MAIN } className={ ({ isActive }) => clsx(styles.tab, isActive && styles.active) }>
                <HomeIcon />
                Главная
            </NavLink>
            {tabs.map((tab) => {
                return tab.title !== "Настройки*" ? (
                    <NavLink key={ tab.id } to={ `/models/${tab.id}` } className={ ({ isActive }) => clsx(styles.tab, isActive && styles.active) }>
                        <tab.model.icon />
                        {tab.title}
                        <Cancel
                            className={ styles.cancelIcon }
                            onClick={ (e) => {
                                e.preventDefault();
                                e.stopPropagation();
                                removeTab(tab);
                            } }
                        />
                    </NavLink>
                ) : (
                    <NavLink key={ tab.id } to={ "/settings" } className={ ({ isActive }) => clsx(styles.tab, isActive && styles.active) }>
                        <tab.model.icon />
                        {tab.title}
                        <Cancel
                            className={ styles.cancelIcon }
                            onClick={ (e) => {
                                e.preventDefault();
                                e.stopPropagation();
                                removeTab(tab);
                            } }
                        />
                    </NavLink> );
            })}

            <DropdownMenu.Root>
                <DropdownMenu.Trigger className={ styles.plusButton }>
                    <Plus />
                </DropdownMenu.Trigger>

                <DropdownMenu.Content className={ styles.dropdown }>
                    <DropdownMenu.Group>
                        {models.map((model) => {
                            return (
                                <DropdownMenu.Item key={ model.type } className={ styles.item } onClick={ () => addTab(model) }>
                                    <model.icon />
                                    {model.type}
                                </DropdownMenu.Item>
                            );
                        })}
                    </DropdownMenu.Group>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </div>
    );
}

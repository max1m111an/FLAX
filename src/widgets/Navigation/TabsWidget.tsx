import { useSyncExternalStore } from "react";
import { ROUTES } from "@/configs/RoutesConst.ts";
import { NavLink } from "react-router-dom";
import HomeIcon from "@/assets/svg/Home.svg?react";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import Plus from "@/assets/svg/Plus.svg?react";
import Cancel from "@/assets/svg/Cancel.svg?react";
import { useTabs } from "@/context/TabsContext.tsx";
import { isDirty, stripDirtyStar, subscribeDirty, getDirtySnapshot } from "@/services/dirtyState.ts";
import clsx from "clsx";
import styles from "./TabsWidget.module.scss";
import { AppTab } from "@/types/Tabs.ts";
import { models } from "@/data/Models.ts";


export default function TabsWidget() {
    const { tabs, addTab, removeTab } = useTabs();
    useSyncExternalStore(subscribeDirty, getDirtySnapshot, getDirtySnapshot);

    const displayTitle = (tab: AppTab) => {
        const base = stripDirtyStar(tab.title);
        return isDirty(tab.id) ? `${base}*` : base;
    };


    return (
        <div className={ styles.tabsWrapper }>
            <NavLink to={ ROUTES.MAIN } className={ ({ isActive }) => clsx(styles.tab, isActive && styles.active) }>
                <HomeIcon />
                Главная
            </NavLink>
            {tabs.map((tab) => {
                return tab.type !== "SETTINGS" ? (
                    <NavLink key={ tab.id } to={ `/models/${tab.id}` } className={ ({ isActive }) => clsx(styles.tab, isActive && styles.active) }>
                        <tab.icon />
                        {displayTitle(tab)}
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
                        <tab.icon />
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
                                <DropdownMenu.Item key={ model.title } className={ styles.item } onClick={ () => addTab(model) }>
                                    <model.icon />
                                    {model.title}
                                </DropdownMenu.Item>
                            );
                        })}
                    </DropdownMenu.Group>
                </DropdownMenu.Content>
            </DropdownMenu.Root>
        </div>
    );
}

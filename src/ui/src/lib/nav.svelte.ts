export interface NavItem {
	label: string;
	icon: string;
	page: string;
}

export const navigationItems: NavItem[] = [
	{
		label: 'User List',
		icon: 'Users',
		page: 'user-list'
	},
	{
		label: 'Create New User',
		icon: 'Plus',
		page: 'new-user'
	},
	{
		label: 'Settings',
		icon: 'Settings',
		page: 'settings'
	}
];

export const navigationState = $state({
	activePage: 'user-list',
	isSidebarExpanded: false
});

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
	}
];

export interface User {
	id: number;
	name: string;
}

export const navigationState = $state({
	activePage: 'user-list',
	isSidebarExpanded: false,
	activeUserEditUser: null as User | null
});

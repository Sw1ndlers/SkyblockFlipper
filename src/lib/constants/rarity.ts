export enum Rarity {
	Common,
	Uncommon,
	Rare,
	Epic,
	Legendary,
	Mythic,
	Special,
	VerySpecial,
	Supreme
}

type RarityColors = {
	[K in Rarity]: string;
};

export const rarityColors: RarityColors = {
	[Rarity.Common]: '#ffffff',
	[Rarity.Uncommon]: '#5555ff',
	[Rarity.Rare]: '#5555FF',
	[Rarity.Epic]: '#aa00aa',
	[Rarity.Legendary]: '#ffaa00',
	[Rarity.Mythic]: '#ff55ff',
	[Rarity.Special]: '#ff5555',
	[Rarity.VerySpecial]: '#ff5555',
	[Rarity.Supreme]: '#55ffff'
};

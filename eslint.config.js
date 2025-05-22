import { general } from "@mirite/eslint-config-mirite";

export default [
	...general,
	{
		rules: {
			"jsdoc/check-tag-names": "off",
			"jsdoc/no-types": "off",
			"jsdoc/require-param-type": "error",
			"jsdoc/require-returns-type": "error",
		},
	},
	{
		ignores: ["**/pkg/**"],
	},
];

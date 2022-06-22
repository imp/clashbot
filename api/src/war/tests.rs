use serde_json as json;

use super::*;

const WARLOG_JSON: &str = r##"
{
  "items": [
    {
      "result": "lose",
      "endTime": "20220620T181736.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 29,
        "stars": 39,
        "destructionPercentage": 86.4,
        "expEarned": 186
      },
      "opponent": {
        "tag": "#Y9U29LVV",
        "name": "iran WOLF baX",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/SgdLPD2CPthk0dcL2YkQZXBTNCdC3W3SC9hshzoZTOc.png",
          "large": "https://api-assets.clashofclans.com/badges/512/SgdLPD2CPthk0dcL2YkQZXBTNCdC3W3SC9hshzoZTOc.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/SgdLPD2CPthk0dcL2YkQZXBTNCdC3W3SC9hshzoZTOc.png"
        },
        "clanLevel": 18,
        "stars": 39,
        "destructionPercentage": 92.333336
      }
    },
    {
      "result": "win",
      "endTime": "20220618T185531.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 29,
        "stars": 37,
        "destructionPercentage": 82.933334,
        "expEarned": 240
      },
      "opponent": {
        "tag": "#VYVJ92G8",
        "name": "صہدفہةّ لتقہينآ",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/gyIltqPDhUO9FFAAVT5YWvaycETt2wlGyr2EPuZTvqo.png",
          "large": "https://api-assets.clashofclans.com/badges/512/gyIltqPDhUO9FFAAVT5YWvaycETt2wlGyr2EPuZTvqo.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/gyIltqPDhUO9FFAAVT5YWvaycETt2wlGyr2EPuZTvqo.png"
        },
        "clanLevel": 21,
        "stars": 35,
        "destructionPercentage": 82
      }
    },
    {
      "result": "lose",
      "endTime": "20220616T184943.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 30,
        "stars": 33,
        "destructionPercentage": 76.13333,
        "expEarned": 191
      },
      "opponent": {
        "tag": "#GQUCGQ2G",
        "name": "Les x clans",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/m5_kh_lqEGY_JVyxOtJiGhHBsaQZocKjARjZAdAeZa4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/m5_kh_lqEGY_JVyxOtJiGhHBsaQZocKjARjZAdAeZa4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/m5_kh_lqEGY_JVyxOtJiGhHBsaQZocKjARjZAdAeZa4.png"
        },
        "clanLevel": 19,
        "stars": 41,
        "destructionPercentage": 97.46667
      }
    },
    {
      "result": "win",
      "endTime": "20220614T175443.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 29,
        "stars": 37,
        "destructionPercentage": 88.2,
        "expEarned": 234
      },
      "opponent": {
        "tag": "#P0VRR9RQ",
        "name": "zikortus",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KHfmePuMoYfOEnLPXzk1YE2VaNziAtbg93tZt-BfDBY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KHfmePuMoYfOEnLPXzk1YE2VaNziAtbg93tZt-BfDBY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KHfmePuMoYfOEnLPXzk1YE2VaNziAtbg93tZt-BfDBY.png"
        },
        "clanLevel": 20,
        "stars": 31,
        "destructionPercentage": 79.8
      }
    },
    {
      "result": "win",
      "endTime": "20220612T181332.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 30,
        "stars": 39,
        "destructionPercentage": 87.333336,
        "expEarned": 237
      },
      "opponent": {
        "tag": "#29P28CUU9",
        "name": "Horn of Dragon",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/dhY8ZUT84E_lUwfVCacBoT_38qaj3ry4RPuEH8K3ldA.png",
          "large": "https://api-assets.clashofclans.com/badges/512/dhY8ZUT84E_lUwfVCacBoT_38qaj3ry4RPuEH8K3ldA.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/dhY8ZUT84E_lUwfVCacBoT_38qaj3ry4RPuEH8K3ldA.png"
        },
        "clanLevel": 16,
        "stars": 35,
        "destructionPercentage": 83.333336
      }
    },
    {
      "result": null,
      "endTime": "20220610T075727.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 103,
        "stars": 310,
        "destructionPercentage": 589.13336,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 141,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20220531T183838.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 39,
        "stars": 52,
        "destructionPercentage": 91.6,
        "expEarned": 238
      },
      "opponent": {
        "tag": "#29PP8PLQR",
        "name": ".......",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/jOHpjRSSBEGI8Y8TCZsM1i9fZNRps8lydVklLe1F19Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/jOHpjRSSBEGI8Y8TCZsM1i9fZNRps8lydVklLe1F19Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/jOHpjRSSBEGI8Y8TCZsM1i9fZNRps8lydVklLe1F19Q.png"
        },
        "clanLevel": 13,
        "stars": 56,
        "destructionPercentage": 97.25
      }
    },
    {
      "result": "win",
      "endTime": "20220529T185544.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 20,
        "stars": 26,
        "destructionPercentage": 90.3,
        "expEarned": 182
      },
      "opponent": {
        "tag": "#RQ9CCCPG",
        "name": "Adultes & Co",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/XJRdG1IhAsJnthyfU8pbh6HIEaduOD2xyKUBR7bh4xM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/XJRdG1IhAsJnthyfU8pbh6HIEaduOD2xyKUBR7bh4xM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/XJRdG1IhAsJnthyfU8pbh6HIEaduOD2xyKUBR7bh4xM.png"
        },
        "clanLevel": 22,
        "stars": 24,
        "destructionPercentage": 88.7
      }
    },
    {
      "result": "lose",
      "endTime": "20220527T175924.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 28,
        "stars": 38,
        "destructionPercentage": 86.4,
        "expEarned": 186
      },
      "opponent": {
        "tag": "#GYRYU2GV",
        "name": "Trendy Tamizhan",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/LN5T_TfXKTqOb9C69WM83-9oYD-WMka8UWRjWLgWz2A.png",
          "large": "https://api-assets.clashofclans.com/badges/512/LN5T_TfXKTqOb9C69WM83-9oYD-WMka8UWRjWLgWz2A.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/LN5T_TfXKTqOb9C69WM83-9oYD-WMka8UWRjWLgWz2A.png"
        },
        "clanLevel": 18,
        "stars": 41,
        "destructionPercentage": 94.933334
      }
    },
    {
      "result": "win",
      "endTime": "20220525T181831.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 28,
        "stars": 38,
        "destructionPercentage": 87.6,
        "expEarned": 237
      },
      "opponent": {
        "tag": "#2GR2GY9J",
        "name": "МЕДВЕДИ.777",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/gUb3atSqVP0FnuglAX4fLQxQf432LHrsPa89EU7Ixeo.png",
          "large": "https://api-assets.clashofclans.com/badges/512/gUb3atSqVP0FnuglAX4fLQxQf432LHrsPa89EU7Ixeo.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/gUb3atSqVP0FnuglAX4fLQxQf432LHrsPa89EU7Ixeo.png"
        },
        "clanLevel": 21,
        "stars": 31,
        "destructionPercentage": 72.26667
      }
    },
    {
      "result": "win",
      "endTime": "20220523T180531.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 29,
        "stars": 35,
        "destructionPercentage": 80.73333,
        "expEarned": 241
      },
      "opponent": {
        "tag": "#2PY9LYQ02",
        "name": "German_ISF@SWC",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/oSdF90DRgC4iv-El41atfduhnGzKB_ZhxOKLAitDMtQ.png",
          "large": "https://api-assets.clashofclans.com/badges/512/oSdF90DRgC4iv-El41atfduhnGzKB_ZhxOKLAitDMtQ.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/oSdF90DRgC4iv-El41atfduhnGzKB_ZhxOKLAitDMtQ.png"
        },
        "clanLevel": 15,
        "stars": 33,
        "destructionPercentage": 78.86667
      }
    },
    {
      "result": "win",
      "endTime": "20220521T182720.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 24,
        "stars": 34,
        "destructionPercentage": 81.46667,
        "expEarned": 244
      },
      "opponent": {
        "tag": "#9CURRYUV",
        "name": "SWE",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/6J6puPE9DfGRztF08u4sZdgVwKm0s4XgD988dyxr-jo.png",
          "large": "https://api-assets.clashofclans.com/badges/512/6J6puPE9DfGRztF08u4sZdgVwKm0s4XgD988dyxr-jo.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/6J6puPE9DfGRztF08u4sZdgVwKm0s4XgD988dyxr-jo.png"
        },
        "clanLevel": 17,
        "stars": 32,
        "destructionPercentage": 80.4
      }
    },
    {
      "result": "lose",
      "endTime": "20220519T183701.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 39,
        "stars": 46,
        "destructionPercentage": 80.8,
        "expEarned": 241
      },
      "opponent": {
        "tag": "#22Q9289GY",
        "name": "دراويش سامبا",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KyMJggibyWx5xUb-ompllreXZciLVkiukjnJSAJNMPg.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KyMJggibyWx5xUb-ompllreXZciLVkiukjnJSAJNMPg.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KyMJggibyWx5xUb-ompllreXZciLVkiukjnJSAJNMPg.png"
        },
        "clanLevel": 17,
        "stars": 51,
        "destructionPercentage": 93
      }
    },
    {
      "result": "win",
      "endTime": "20220517T180612.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 40,
        "stars": 51,
        "destructionPercentage": 87.75,
        "expEarned": 286
      },
      "opponent": {
        "tag": "#LYVCLPQL",
        "name": "عاصفة الحرب",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/aIoVQpzDf7scqm0xRTAT6yluUI1ey4DB6O4J_SX1umg.png",
          "large": "https://api-assets.clashofclans.com/badges/512/aIoVQpzDf7scqm0xRTAT6yluUI1ey4DB6O4J_SX1umg.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/aIoVQpzDf7scqm0xRTAT6yluUI1ey4DB6O4J_SX1umg.png"
        },
        "clanLevel": 20,
        "stars": 42,
        "destructionPercentage": 74.45
      }
    },
    {
      "result": "lose",
      "endTime": "20220515T090622.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 39,
        "stars": 54,
        "destructionPercentage": 89.35,
        "expEarned": 227
      },
      "opponent": {
        "tag": "#LJR92Y09",
        "name": "#include",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/yzEGPFP5T5XQ_GNFE993EMjGEJHFgE0lYrkNvWkGnrw.png",
          "large": "https://api-assets.clashofclans.com/badges/512/yzEGPFP5T5XQ_GNFE993EMjGEJHFgE0lYrkNvWkGnrw.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/yzEGPFP5T5XQ_GNFE993EMjGEJHFgE0lYrkNvWkGnrw.png"
        },
        "clanLevel": 15,
        "stars": 55,
        "destructionPercentage": 94.3
      }
    },
    {
      "result": null,
      "endTime": "20220510T064320.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 104,
        "stars": 296,
        "destructionPercentage": 589.5333,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 155,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20220501T045821.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 36,
        "stars": 45,
        "destructionPercentage": 74.85,
        "expEarned": 241
      },
      "opponent": {
        "tag": "#YQR9G9U0",
        "name": "Aliados GT",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/QAlpJsKFXr9OMf20dXoyWEUkFeUJnhoMOPbgzN0qMl4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/QAlpJsKFXr9OMf20dXoyWEUkFeUJnhoMOPbgzN0qMl4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/QAlpJsKFXr9OMf20dXoyWEUkFeUJnhoMOPbgzN0qMl4.png"
        },
        "clanLevel": 21,
        "stars": 52,
        "destructionPercentage": 90.85
      }
    },
    {
      "result": "win",
      "endTime": "20220429T002428.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGDG4IF7NLg23s_fluhoFcW82OZk1F_78od657nNo-Q.png"
        },
        "clanLevel": 20,
        "attacks": 37,
        "stars": 46,
        "destructionPercentage": 84.85,
        "expEarned": 293
      },
      "opponent": {
        "tag": "#22LGGJQ0Y",
        "name": "quebec gix",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/CA8ZWoWZJHELBBqCiwgITL7uJreCcSB_HZvyZkkN0Go.png",
          "large": "https://api-assets.clashofclans.com/badges/512/CA8ZWoWZJHELBBqCiwgITL7uJreCcSB_HZvyZkkN0Go.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/CA8ZWoWZJHELBBqCiwgITL7uJreCcSB_HZvyZkkN0Go.png"
        },
        "clanLevel": 21,
        "stars": 44,
        "destructionPercentage": 82.95
      }
    },
    {
      "result": "win",
      "endTime": "20220426T214047.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 40,
        "stars": 51,
        "destructionPercentage": 90,
        "expEarned": 285
      },
      "opponent": {
        "tag": "#9GU0UC9J",
        "name": "Fearless Axe",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/nooHhQMrZil1RH1v000zhXRtEeBaHaFGyBezzaidWm4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/nooHhQMrZil1RH1v000zhXRtEeBaHaFGyBezzaidWm4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/nooHhQMrZil1RH1v000zhXRtEeBaHaFGyBezzaidWm4.png"
        },
        "clanLevel": 19,
        "stars": 48,
        "destructionPercentage": 86.1
      }
    },
    {
      "result": "win",
      "endTime": "20220424T194827.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 40,
        "stars": 53,
        "destructionPercentage": 91.35,
        "expEarned": 283
      },
      "opponent": {
        "tag": "#2980GU8UC",
        "name": "old school ☆☆☆☆",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/ZkIycFr8m8-to45HQera_4VHXb4br4Y5I5jWgzWR-OE.png",
          "large": "https://api-assets.clashofclans.com/badges/512/ZkIycFr8m8-to45HQera_4VHXb4br4Y5I5jWgzWR-OE.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/ZkIycFr8m8-to45HQera_4VHXb4br4Y5I5jWgzWR-OE.png"
        },
        "clanLevel": 12,
        "stars": 45,
        "destructionPercentage": 76.6
      }
    },
    {
      "result": "lose",
      "endTime": "20220422T200906.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 36,
        "stars": 52,
        "destructionPercentage": 86.35,
        "expEarned": 229
      },
      "opponent": {
        "tag": "#RU80YGY",
        "name": "C.K.E.A.",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/2PP9uyYE8L0xPr-W1ak_Ly5n0UD_eO6awHo37gUrYeY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/2PP9uyYE8L0xPr-W1ak_Ly5n0UD_eO6awHo37gUrYeY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/2PP9uyYE8L0xPr-W1ak_Ly5n0UD_eO6awHo37gUrYeY.png"
        },
        "clanLevel": 21,
        "stars": 58,
        "destructionPercentage": 98.7
      }
    },
    {
      "result": "win",
      "endTime": "20220420T175959.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 24,
        "stars": 37,
        "destructionPercentage": 84,
        "expEarned": 235
      },
      "opponent": {
        "tag": "#YUUQUCV8",
        "name": "Днестровск ПМР",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/dJbdTf0ORivJ_QZAaL2kSilp03bQQo-Tw-bCVkAX-rA.png",
          "large": "https://api-assets.clashofclans.com/badges/512/dJbdTf0ORivJ_QZAaL2kSilp03bQQo-Tw-bCVkAX-rA.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/dJbdTf0ORivJ_QZAaL2kSilp03bQQo-Tw-bCVkAX-rA.png"
        },
        "clanLevel": 20,
        "stars": 36,
        "destructionPercentage": 80.46667
      }
    },
    {
      "result": "lose",
      "endTime": "20220418T042458.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 28,
        "stars": 31,
        "destructionPercentage": 72.066666,
        "expEarned": 190
      },
      "opponent": {
        "tag": "#29Q99VPUC",
        "name": "THE PUNISHER",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/G-K8zra9cwJcvXtt89hV5wQLivSXxUSKZh80a2mIMiY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/G-K8zra9cwJcvXtt89hV5wQLivSXxUSKZh80a2mIMiY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/G-K8zra9cwJcvXtt89hV5wQLivSXxUSKZh80a2mIMiY.png"
        },
        "clanLevel": 13,
        "stars": 41,
        "destructionPercentage": 93.933334
      }
    },
    {
      "result": "lose",
      "endTime": "20220416T032926.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 36,
        "stars": 51,
        "destructionPercentage": 89.4,
        "expEarned": 230
      },
      "opponent": {
        "tag": "#2YJG90R9L",
        "name": "Philippines",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/fLkJwNz4cqCS4Tstna-4F4VUsSNpZw02dljI592ecFg.png",
          "large": "https://api-assets.clashofclans.com/badges/512/fLkJwNz4cqCS4Tstna-4F4VUsSNpZw02dljI592ecFg.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/fLkJwNz4cqCS4Tstna-4F4VUsSNpZw02dljI592ecFg.png"
        },
        "clanLevel": 11,
        "stars": 52,
        "destructionPercentage": 89
      }
    },
    {
      "result": "win",
      "endTime": "20220414T024609.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 40,
        "stars": 53,
        "destructionPercentage": 93.9,
        "expEarned": 277
      },
      "opponent": {
        "tag": "#29LQQ8U2C",
        "name": "A Arte da Guerr",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/I8we-LqZafK7py1EUYSDZTiq-OEyeD05mEobDJJ6ytI.png",
          "large": "https://api-assets.clashofclans.com/badges/512/I8we-LqZafK7py1EUYSDZTiq-OEyeD05mEobDJJ6ytI.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/I8we-LqZafK7py1EUYSDZTiq-OEyeD05mEobDJJ6ytI.png"
        },
        "clanLevel": 13,
        "stars": 53,
        "destructionPercentage": 91.05
      }
    },
    {
      "result": null,
      "endTime": "20220411T033252.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 102,
        "stars": 293,
        "destructionPercentage": 583.06665,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 146,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20220402T205143.000Z",
      "teamSize": 25,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 46,
        "stars": 64,
        "destructionPercentage": 86.52,
        "expEarned": 281
      },
      "opponent": {
        "tag": "#YRCCV8YL",
        "name": "LEGEND OF CLASH",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/D7hgHvfnkj3GH7GIcPQbferdttPANJ36d8S5TPzNFqg.png",
          "large": "https://api-assets.clashofclans.com/badges/512/D7hgHvfnkj3GH7GIcPQbferdttPANJ36d8S5TPzNFqg.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/D7hgHvfnkj3GH7GIcPQbferdttPANJ36d8S5TPzNFqg.png"
        },
        "clanLevel": 22,
        "stars": 67,
        "destructionPercentage": 91.48
      }
    },
    {
      "result": "win",
      "endTime": "20220330T224244.000Z",
      "teamSize": 25,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 46,
        "stars": 63,
        "destructionPercentage": 88.4,
        "expEarned": 329
      },
      "opponent": {
        "tag": "#QRYY0GJ2",
        "name": "kame house",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/u0xHVNFoR582piz0N0p8tzcVyKSJuAIXZOmUkGIBu6o.png",
          "large": "https://api-assets.clashofclans.com/badges/512/u0xHVNFoR582piz0N0p8tzcVyKSJuAIXZOmUkGIBu6o.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/u0xHVNFoR582piz0N0p8tzcVyKSJuAIXZOmUkGIBu6o.png"
        },
        "clanLevel": 19,
        "stars": 60,
        "destructionPercentage": 89.6
      }
    },
    {
      "result": "lose",
      "endTime": "20220328T193021.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 29,
        "stars": 37,
        "destructionPercentage": 82.13333,
        "expEarned": 180
      },
      "opponent": {
        "tag": "#2990QPPV",
        "name": "Reis de Sangue",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/Z8Z1Er0uyqqaTAyDqsIh21SoIfs9WWFvmUgw8vH8jcw.png",
          "large": "https://api-assets.clashofclans.com/badges/512/Z8Z1Er0uyqqaTAyDqsIh21SoIfs9WWFvmUgw8vH8jcw.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/Z8Z1Er0uyqqaTAyDqsIh21SoIfs9WWFvmUgw8vH8jcw.png"
        },
        "clanLevel": 20,
        "stars": 37,
        "destructionPercentage": 88.066666
      }
    },
    {
      "result": "lose",
      "endTime": "20220320T073123.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 25,
        "stars": 32,
        "destructionPercentage": 80.73333,
        "expEarned": 183
      },
      "opponent": {
        "tag": "#Y8V9JU8P",
        "name": "هراتی ها",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/UUU2n22guaNvPh1PyO5oIOhn8NiEIFg5phRaRnmJFgE.png",
          "large": "https://api-assets.clashofclans.com/badges/512/UUU2n22guaNvPh1PyO5oIOhn8NiEIFg5phRaRnmJFgE.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/UUU2n22guaNvPh1PyO5oIOhn8NiEIFg5phRaRnmJFgE.png"
        },
        "clanLevel": 18,
        "stars": 37,
        "destructionPercentage": 88.86667
      }
    },
    {
      "result": "win",
      "endTime": "20220311T041524.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 102,
        "stars": 300,
        "destructionPercentage": 599.73334,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 155,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20220220T232729.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 18,
        "stars": 26,
        "destructionPercentage": 87,
        "expEarned": 175
      },
      "opponent": {
        "tag": "#22UGJV9PJ",
        "name": "RyceZ",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/IHa1OpfbdgUvtdjULgQnvNWLW49ZKSTFRJVr2ianrB4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/IHa1OpfbdgUvtdjULgQnvNWLW49ZKSTFRJVr2ianrB4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/IHa1OpfbdgUvtdjULgQnvNWLW49ZKSTFRJVr2ianrB4.png"
        },
        "clanLevel": 12,
        "stars": 19,
        "destructionPercentage": 70
      }
    },
    {
      "result": "lose",
      "endTime": "20220218T015248.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 19,
        "stars": 27,
        "destructionPercentage": 92.9,
        "expEarned": 130
      },
      "opponent": {
        "tag": "#UPGV82YR",
        "name": "vinh long no 2",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KaNRQ61tCONAcato_mE41wX2O16QiSyNcHeGYlKBNjg.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KaNRQ61tCONAcato_mE41wX2O16QiSyNcHeGYlKBNjg.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KaNRQ61tCONAcato_mE41wX2O16QiSyNcHeGYlKBNjg.png"
        },
        "clanLevel": 16,
        "stars": 27,
        "destructionPercentage": 96.5
      }
    },
    {
      "result": "win",
      "endTime": "20220215T020231.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 22,
        "stars": 38,
        "destructionPercentage": 87.2,
        "expEarned": 229
      },
      "opponent": {
        "tag": "#28CLUVJQL",
        "name": "Legendarios",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/gMIravKwMcg2IAXhID57SHsND3zf_KfssQ67U4y9Pv4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/gMIravKwMcg2IAXhID57SHsND3zf_KfssQ67U4y9Pv4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/gMIravKwMcg2IAXhID57SHsND3zf_KfssQ67U4y9Pv4.png"
        },
        "clanLevel": 15,
        "stars": 30,
        "destructionPercentage": 72.6
      }
    },
    {
      "result": "tie",
      "endTime": "20220208T050716.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 73,
        "stars": 129,
        "destructionPercentage": 320.13333,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 163,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20220129T225129.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 22,
        "stars": 39,
        "destructionPercentage": 89.26667,
        "expEarned": 220
      },
      "opponent": {
        "tag": "#292YJ988",
        "name": "The Lake Show",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/a8OBgscIkhsHgGKBWNhOuYTgnOykWCxSjceW-eEdll8.png",
          "large": "https://api-assets.clashofclans.com/badges/512/a8OBgscIkhsHgGKBWNhOuYTgnOykWCxSjceW-eEdll8.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/a8OBgscIkhsHgGKBWNhOuYTgnOykWCxSjceW-eEdll8.png"
        },
        "clanLevel": 20,
        "stars": 34,
        "destructionPercentage": 84.6
      }
    },
    {
      "result": "win",
      "endTime": "20220125T000725.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 22,
        "stars": 35,
        "destructionPercentage": 85.2,
        "expEarned": 225
      },
      "opponent": {
        "tag": "#JR0Y0PJG",
        "name": "暗夜精灵",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/ZEdaEQ_F5O_8puXQM2BzRF-hTE3PM_ywnoV-YmkHuSo.png",
          "large": "https://api-assets.clashofclans.com/badges/512/ZEdaEQ_F5O_8puXQM2BzRF-hTE3PM_ywnoV-YmkHuSo.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/ZEdaEQ_F5O_8puXQM2BzRF-hTE3PM_ywnoV-YmkHuSo.png"
        },
        "clanLevel": 17,
        "stars": 31,
        "destructionPercentage": 83.13333
      }
    },
    {
      "result": "lose",
      "endTime": "20220122T215905.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 25,
        "stars": 35,
        "destructionPercentage": 84,
        "expEarned": 179
      },
      "opponent": {
        "tag": "#2992J0G9Y",
        "name": "القيصر",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/q7HhxbgARi5iJSuoXHVM5M-hmF-_9fFR9pvqNl4Pi40.png",
          "large": "https://api-assets.clashofclans.com/badges/512/q7HhxbgARi5iJSuoXHVM5M-hmF-_9fFR9pvqNl4Pi40.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/q7HhxbgARi5iJSuoXHVM5M-hmF-_9fFR9pvqNl4Pi40.png"
        },
        "clanLevel": 17,
        "stars": 39,
        "destructionPercentage": 91.8
      }
    },
    {
      "result": "lose",
      "endTime": "20220115T003258.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 18,
        "stars": 23,
        "destructionPercentage": 79.9,
        "expEarned": 134
      },
      "opponent": {
        "tag": "#YRGG2LGJ",
        "name": "Señor Beaver",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/Tadp6qEVn56j4OYAcoewOIrhTnGfR0WsAV862IVtodU.png",
          "large": "https://api-assets.clashofclans.com/badges/512/Tadp6qEVn56j4OYAcoewOIrhTnGfR0WsAV862IVtodU.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/Tadp6qEVn56j4OYAcoewOIrhTnGfR0WsAV862IVtodU.png"
        },
        "clanLevel": 19,
        "stars": 27,
        "destructionPercentage": 95.6
      }
    },
    {
      "result": "win",
      "endTime": "20220110T032101.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 103,
        "stars": 304,
        "destructionPercentage": 568.13336,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 124,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20211230T014520.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 19,
        "stars": 24,
        "destructionPercentage": 87.9,
        "expEarned": 185
      },
      "opponent": {
        "tag": "#9UVLUJUJ",
        "name": "Chupa Manga",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/YGJfzJ-fYyiVf-7xzSPERMMfrE_e39E4c1pQ7xhh0tM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/YGJfzJ-fYyiVf-7xzSPERMMfrE_e39E4c1pQ7xhh0tM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/YGJfzJ-fYyiVf-7xzSPERMMfrE_e39E4c1pQ7xhh0tM.png"
        },
        "clanLevel": 18,
        "stars": 0,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20211226T042202.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 13,
        "stars": 21,
        "destructionPercentage": 75.5,
        "expEarned": 124
      },
      "opponent": {
        "tag": "#2UV2UC9J",
        "name": "Az Wolfs",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/Rk7vZMkuGtmHqKAlo95cy9LFe8FLZxFPCHRumV4LAbY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/Rk7vZMkuGtmHqKAlo95cy9LFe8FLZxFPCHRumV4LAbY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/Rk7vZMkuGtmHqKAlo95cy9LFe8FLZxFPCHRumV4LAbY.png"
        },
        "clanLevel": 18,
        "stars": 21,
        "destructionPercentage": 77.8
      }
    },
    {
      "result": "lose",
      "endTime": "20211222T024923.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 27,
        "stars": 35,
        "destructionPercentage": 82.86667,
        "expEarned": 184
      },
      "opponent": {
        "tag": "#2Y00GYPGG",
        "name": "THE ONE FAMILY",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/trobAWhLjzc0Lfv1PoppzYo9Tiwv-cCBF-4hHBiNqH8.png",
          "large": "https://api-assets.clashofclans.com/badges/512/trobAWhLjzc0Lfv1PoppzYo9Tiwv-cCBF-4hHBiNqH8.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/trobAWhLjzc0Lfv1PoppzYo9Tiwv-cCBF-4hHBiNqH8.png"
        },
        "clanLevel": 10,
        "stars": 37,
        "destructionPercentage": 91
      }
    },
    {
      "result": "lose",
      "endTime": "20211219T190331.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 30,
        "stars": 36,
        "destructionPercentage": 81.4,
        "expEarned": 181
      },
      "opponent": {
        "tag": "#908VCYQU",
        "name": "iran 011",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/Cg6wqY6riPRehyTsw87lztVZvIaQgS4yPQlAuRmR7bc.png",
          "large": "https://api-assets.clashofclans.com/badges/512/Cg6wqY6riPRehyTsw87lztVZvIaQgS4yPQlAuRmR7bc.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/Cg6wqY6riPRehyTsw87lztVZvIaQgS4yPQlAuRmR7bc.png"
        },
        "clanLevel": 16,
        "stars": 36,
        "destructionPercentage": 88.666664
      }
    },
    {
      "result": "win",
      "endTime": "20211217T021431.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 26,
        "stars": 36,
        "destructionPercentage": 84.73333,
        "expEarned": 237
      },
      "opponent": {
        "tag": "#22VV2G02C",
        "name": "DARE DEVILS",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/--MCpMoQD5ztoDve09pv3jj8z4y5qGxCKsyzXnc3VwY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/--MCpMoQD5ztoDve09pv3jj8z4y5qGxCKsyzXnc3VwY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/--MCpMoQD5ztoDve09pv3jj8z4y5qGxCKsyzXnc3VwY.png"
        },
        "clanLevel": 13,
        "stars": 34,
        "destructionPercentage": 83.2
      }
    },
    {
      "result": "lose",
      "endTime": "20211214T232715.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 26,
        "stars": 35,
        "destructionPercentage": 82.26667,
        "expEarned": 181
      },
      "opponent": {
        "tag": "#UGPUCJQJ",
        "name": "AE Đoàn Kết",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WnhVwfMZuY6siCdrBYzN4xbWHdxdKHkjMIZVwNchW_8.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WnhVwfMZuY6siCdrBYzN4xbWHdxdKHkjMIZVwNchW_8.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WnhVwfMZuY6siCdrBYzN4xbWHdxdKHkjMIZVwNchW_8.png"
        },
        "clanLevel": 19,
        "stars": 36,
        "destructionPercentage": 84.2
      }
    },
    {
      "result": "win",
      "endTime": "20211212T061224.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 26,
        "stars": 33,
        "destructionPercentage": 84.53333,
        "expEarned": 232
      },
      "opponent": {
        "tag": "#YY2QLUUL",
        "name": "平凡的惊喜",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/vCXFfSjATINe2CPJbVsoMhB-Z38e9-lu093nNp3XHF8.png",
          "large": "https://api-assets.clashofclans.com/badges/512/vCXFfSjATINe2CPJbVsoMhB-Z38e9-lu093nNp3XHF8.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/vCXFfSjATINe2CPJbVsoMhB-Z38e9-lu093nNp3XHF8.png"
        },
        "clanLevel": 18,
        "stars": 16,
        "destructionPercentage": 58
      }
    },
    {
      "result": "tie",
      "endTime": "20211209T214800.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 103,
        "stars": 168,
        "destructionPercentage": 394.73334,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 209,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20211201T201121.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 18,
        "stars": 27,
        "destructionPercentage": 93.1,
        "expEarned": 184
      },
      "opponent": {
        "tag": "#QYV9PUCC",
        "name": "سرزمین عجایب",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/8igLsY7rfmiXhdf5WtIc90P4xRhZebiIM7x-pdWG0WU.png",
          "large": "https://api-assets.clashofclans.com/badges/512/8igLsY7rfmiXhdf5WtIc90P4xRhZebiIM7x-pdWG0WU.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/8igLsY7rfmiXhdf5WtIc90P4xRhZebiIM7x-pdWG0WU.png"
        },
        "clanLevel": 19,
        "stars": 11,
        "destructionPercentage": 60.7
      }
    },
    {
      "result": "lose",
      "endTime": "20211129T210342.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 17,
        "stars": 25,
        "destructionPercentage": 86.8,
        "expEarned": 131
      },
      "opponent": {
        "tag": "#GQPPJ8J",
        "name": "5ème dimension",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/nZom6E988d-NKJEUEW7E1R_8eQjl0Utz7s_kN8AlEqo.png",
          "large": "https://api-assets.clashofclans.com/badges/512/nZom6E988d-NKJEUEW7E1R_8eQjl0Utz7s_kN8AlEqo.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/nZom6E988d-NKJEUEW7E1R_8eQjl0Utz7s_kN8AlEqo.png"
        },
        "clanLevel": 21,
        "stars": 26,
        "destructionPercentage": 91.4
      }
    },
    {
      "result": "lose",
      "endTime": "20211127T205806.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 19,
        "stars": 22,
        "destructionPercentage": 75.3,
        "expEarned": 122
      },
      "opponent": {
        "tag": "#CYQYYYRY",
        "name": "REAL FRIENDSHIP",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/5TzKtdpHVe496umZ-rSjADAH9z8dkQUvGsmAMPlo2iQ.png",
          "large": "https://api-assets.clashofclans.com/badges/512/5TzKtdpHVe496umZ-rSjADAH9z8dkQUvGsmAMPlo2iQ.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/5TzKtdpHVe496umZ-rSjADAH9z8dkQUvGsmAMPlo2iQ.png"
        },
        "clanLevel": 18,
        "stars": 28,
        "destructionPercentage": 95
      }
    },
    {
      "result": "lose",
      "endTime": "20211125T194038.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 18,
        "stars": 25,
        "destructionPercentage": 88.1,
        "expEarned": 129
      },
      "opponent": {
        "tag": "#YCG9J2L",
        "name": "Troublmaker",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/pO5TQBRlXCw63FWOGbBbvCHjH89Tnh_E-pkzzALQHjQ.png",
          "large": "https://api-assets.clashofclans.com/badges/512/pO5TQBRlXCw63FWOGbBbvCHjH89Tnh_E-pkzzALQHjQ.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/pO5TQBRlXCw63FWOGbBbvCHjH89Tnh_E-pkzzALQHjQ.png"
        },
        "clanLevel": 20,
        "stars": 27,
        "destructionPercentage": 97.1
      }
    },
    {
      "result": "win",
      "endTime": "20211122T193054.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 18,
        "stars": 28,
        "destructionPercentage": 95.5,
        "expEarned": 182
      },
      "opponent": {
        "tag": "#PVURVCY",
        "name": "L'empire royal",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/z37GC8mxa73TFIOoJDGjMB0MtlAGYh5rVvVmvM9o_aY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/z37GC8mxa73TFIOoJDGjMB0MtlAGYh5rVvVmvM9o_aY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/z37GC8mxa73TFIOoJDGjMB0MtlAGYh5rVvVmvM9o_aY.png"
        },
        "clanLevel": 20,
        "stars": 23,
        "destructionPercentage": 89.4
      }
    },
    {
      "result": "win",
      "endTime": "20211120T200028.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 19,
        "stars": 28,
        "destructionPercentage": 91.5,
        "expEarned": 173
      },
      "opponent": {
        "tag": "#28GY9RG88",
        "name": "Jutland vikings",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KadS0VLw9WieZrIW-7Bx6Ly2QJhUsDPxexkb0UsfoyE.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KadS0VLw9WieZrIW-7Bx6Ly2QJhUsDPxexkb0UsfoyE.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KadS0VLw9WieZrIW-7Bx6Ly2QJhUsDPxexkb0UsfoyE.png"
        },
        "clanLevel": 14,
        "stars": 19,
        "destructionPercentage": 68.4
      }
    },
    {
      "result": "win",
      "endTime": "20211116T205838.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 16,
        "stars": 24,
        "destructionPercentage": 93.1,
        "expEarned": 181
      },
      "opponent": {
        "tag": "#228LYVU9V",
        "name": "TOUGH CLASS",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/77hlCM3267J1MlzEe7yhB_TD1fBrpJ3XOCAIdr5ogW4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/77hlCM3267J1MlzEe7yhB_TD1fBrpJ3XOCAIdr5ogW4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/77hlCM3267J1MlzEe7yhB_TD1fBrpJ3XOCAIdr5ogW4.png"
        },
        "clanLevel": 16,
        "stars": 8,
        "destructionPercentage": 26.3
      }
    },
    {
      "result": "lose",
      "endTime": "20211114T064851.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 28,
        "stars": 32,
        "destructionPercentage": 78.666664,
        "expEarned": 179
      },
      "opponent": {
        "tag": "#RQ9VJRYU",
        "name": "Endless War",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/YlhOqzPXmKUGztQrJjm9nBhZFGim56VNbCUsWCMPQrg.png",
          "large": "https://api-assets.clashofclans.com/badges/512/YlhOqzPXmKUGztQrJjm9nBhZFGim56VNbCUsWCMPQrg.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/YlhOqzPXmKUGztQrJjm9nBhZFGim56VNbCUsWCMPQrg.png"
        },
        "clanLevel": 20,
        "stars": 41,
        "destructionPercentage": 92.26667
      }
    },
    {
      "result": null,
      "endTime": "20211111T034013.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 105,
        "stars": 268,
        "destructionPercentage": 529.8,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 185,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20211030T210312.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 16,
        "stars": 25,
        "destructionPercentage": 93.9,
        "expEarned": 122
      },
      "opponent": {
        "tag": "#QR0PCPYU",
        "name": "Donaresempre",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/-BkXMeFB9iTMQboItmORy0BKUjMrsJ5MPdLaOsg4zng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/-BkXMeFB9iTMQboItmORy0BKUjMrsJ5MPdLaOsg4zng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/-BkXMeFB9iTMQboItmORy0BKUjMrsJ5MPdLaOsg4zng.png"
        },
        "clanLevel": 15,
        "stars": 27,
        "destructionPercentage": 92.9
      }
    },
    {
      "result": null,
      "endTime": "20211009T192648.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 101,
        "stars": 211,
        "destructionPercentage": 482.66666,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 208,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20210922T010628.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 27,
        "stars": 38,
        "destructionPercentage": 90.066666,
        "expEarned": 223
      },
      "opponent": {
        "tag": "#QLP8U8Q",
        "name": "Clania",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/SgdLPD2CPthk0dcL2YkQZXBTNCdC3W3SC9hshzoZTOc.png",
          "large": "https://api-assets.clashofclans.com/badges/512/SgdLPD2CPthk0dcL2YkQZXBTNCdC3W3SC9hshzoZTOc.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/SgdLPD2CPthk0dcL2YkQZXBTNCdC3W3SC9hshzoZTOc.png"
        },
        "clanLevel": 18,
        "stars": 35,
        "destructionPercentage": 84
      }
    },
    {
      "result": "win",
      "endTime": "20210919T221230.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 27,
        "stars": 39,
        "destructionPercentage": 87.8,
        "expEarned": 225
      },
      "opponent": {
        "tag": "#VPVGVGYY",
        "name": "MeNtally ILL",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/GabPFNwy1bFA3_1KJMSWcj4MyUiHycoItQHjFwAurXg.png",
          "large": "https://api-assets.clashofclans.com/badges/512/GabPFNwy1bFA3_1KJMSWcj4MyUiHycoItQHjFwAurXg.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/GabPFNwy1bFA3_1KJMSWcj4MyUiHycoItQHjFwAurXg.png"
        },
        "clanLevel": 19,
        "stars": 37,
        "destructionPercentage": 86
      }
    },
    {
      "result": "win",
      "endTime": "20210917T004235.000Z",
      "teamSize": 20,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 37,
        "stars": 54,
        "destructionPercentage": 92.65,
        "expEarned": 267
      },
      "opponent": {
        "tag": "#28UU298VV",
        "name": "Colombia war",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/1hu15JQG0CuB5VVzc_4YneGi_QeYO9h_GhlE0eoo_2o.png",
          "large": "https://api-assets.clashofclans.com/badges/512/1hu15JQG0CuB5VVzc_4YneGi_QeYO9h_GhlE0eoo_2o.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/1hu15JQG0CuB5VVzc_4YneGi_QeYO9h_GhlE0eoo_2o.png"
        },
        "clanLevel": 12,
        "stars": 40,
        "destructionPercentage": 68
      }
    },
    {
      "result": null,
      "endTime": "20210911T030645.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 102,
        "stars": 193,
        "destructionPercentage": 450.06668,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 202,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20210830T220723.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 26,
        "stars": 37,
        "destructionPercentage": 88.2,
        "expEarned": 172
      },
      "opponent": {
        "tag": "#J98LPPG9",
        "name": "3STARS WARRIORS",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/FQe8g7qcsZJo7clfRn8qpZHfVgAJ0IeegQphZMvRyOo.png",
          "large": "https://api-assets.clashofclans.com/badges/512/FQe8g7qcsZJo7clfRn8qpZHfVgAJ0IeegQphZMvRyOo.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/FQe8g7qcsZJo7clfRn8qpZHfVgAJ0IeegQphZMvRyOo.png"
        },
        "clanLevel": 14,
        "stars": 38,
        "destructionPercentage": 87.86667
      }
    },
    {
      "result": "win",
      "endTime": "20210825T012958.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 15,
        "stars": 25,
        "destructionPercentage": 89.8,
        "expEarned": 181
      },
      "opponent": {
        "tag": "#UCLYRC82",
        "name": "RAY｜小然天子",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/U6eHmrWLnHeffwJpqNBreGhjuMJDePw_ohNmd39cYGE.png",
          "large": "https://api-assets.clashofclans.com/badges/512/U6eHmrWLnHeffwJpqNBreGhjuMJDePw_ohNmd39cYGE.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/U6eHmrWLnHeffwJpqNBreGhjuMJDePw_ohNmd39cYGE.png"
        },
        "clanLevel": 11,
        "stars": 15,
        "destructionPercentage": 60.3
      }
    },
    {
      "result": "lose",
      "endTime": "20210817T230311.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 21,
        "stars": 31,
        "destructionPercentage": 80.8,
        "expEarned": 174
      },
      "opponent": {
        "tag": "#PVGPV2RR",
        "name": "iran darvish",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/IgfhOP6GBkWV-Moug3wlZ0kpS23RsUhXbEYdf3YYG4k.png",
          "large": "https://api-assets.clashofclans.com/badges/512/IgfhOP6GBkWV-Moug3wlZ0kpS23RsUhXbEYdf3YYG4k.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/IgfhOP6GBkWV-Moug3wlZ0kpS23RsUhXbEYdf3YYG4k.png"
        },
        "clanLevel": 14,
        "stars": 35,
        "destructionPercentage": 86.13333
      }
    },
    {
      "result": "win",
      "endTime": "20210814T224110.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/KR0YqIX6crjaGgfN-C3mbnWTLaX5CgzsF0J96v8nDwM.png"
        },
        "clanLevel": 19,
        "attacks": 24,
        "stars": 34,
        "destructionPercentage": 81.666664,
        "expEarned": 224
      },
      "opponent": {
        "tag": "#28Q80PJ9J",
        "name": "Dutch Dragons",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/5z34TcY0VoRHNjIBT9vsFkbmEA3QwdV300Bu1oUZ7lY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/5z34TcY0VoRHNjIBT9vsFkbmEA3QwdV300Bu1oUZ7lY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/5z34TcY0VoRHNjIBT9vsFkbmEA3QwdV300Bu1oUZ7lY.png"
        },
        "clanLevel": 13,
        "stars": 33,
        "destructionPercentage": 84.933334
      }
    },
    {
      "result": "win",
      "endTime": "20210810T052344.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 99,
        "stars": 300,
        "destructionPercentage": 579.8,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 144,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20210721T234525.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 17,
        "stars": 25,
        "destructionPercentage": 90.3,
        "expEarned": 176
      },
      "opponent": {
        "tag": "#QY8VUG8Q",
        "name": "INPERIO ROMANO",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/gJ-IfsWTXSjuGnqAcdcBsTJVEO1etr-v8j2gTXqdpS4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/gJ-IfsWTXSjuGnqAcdcBsTJVEO1etr-v8j2gTXqdpS4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/gJ-IfsWTXSjuGnqAcdcBsTJVEO1etr-v8j2gTXqdpS4.png"
        },
        "clanLevel": 17,
        "stars": 24,
        "destructionPercentage": 84.3
      }
    },
    {
      "result": "lose",
      "endTime": "20210719T011532.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 13,
        "stars": 22,
        "destructionPercentage": 84.9,
        "expEarned": 130
      },
      "opponent": {
        "tag": "#QRYYQ82J",
        "name": "CaiLay FC",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/NmAy9iYLKJ9a6_SjUb69-hxkfmWCNT7rHqH8C07nOK0.png",
          "large": "https://api-assets.clashofclans.com/badges/512/NmAy9iYLKJ9a6_SjUb69-hxkfmWCNT7rHqH8C07nOK0.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/NmAy9iYLKJ9a6_SjUb69-hxkfmWCNT7rHqH8C07nOK0.png"
        },
        "clanLevel": 16,
        "stars": 27,
        "destructionPercentage": 96.5
      }
    },
    {
      "result": "lose",
      "endTime": "20210715T220651.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 16,
        "stars": 22,
        "destructionPercentage": 82.3,
        "expEarned": 129
      },
      "opponent": {
        "tag": "#P9PUV8Y8",
        "name": "صيادله وبايلوجي",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/doJ8o2lsMo8S1bjvj0Y_iDPKZvJNF1uudiaiZZYj5wM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/doJ8o2lsMo8S1bjvj0Y_iDPKZvJNF1uudiaiZZYj5wM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/doJ8o2lsMo8S1bjvj0Y_iDPKZvJNF1uudiaiZZYj5wM.png"
        },
        "clanLevel": 14,
        "stars": 29,
        "destructionPercentage": 99.9
      }
    },
    {
      "result": "lose",
      "endTime": "20210713T215605.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 16,
        "stars": 24,
        "destructionPercentage": 87.3,
        "expEarned": 127
      },
      "opponent": {
        "tag": "#YYUUR0JP",
        "name": "Gtg bye",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/TVOB_bMFFm6357nnuYgDkfTC0BKN4hrIUfBhg6oMTLw.png",
          "large": "https://api-assets.clashofclans.com/badges/512/TVOB_bMFFm6357nnuYgDkfTC0BKN4hrIUfBhg6oMTLw.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/TVOB_bMFFm6357nnuYgDkfTC0BKN4hrIUfBhg6oMTLw.png"
        },
        "clanLevel": 18,
        "stars": 27,
        "destructionPercentage": 98.5
      }
    },
    {
      "result": null,
      "endTime": "20210710T005615.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 101,
        "stars": 214,
        "destructionPercentage": 470.33334,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 196,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20210630T230310.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 17,
        "stars": 21,
        "destructionPercentage": 75.8,
        "expEarned": 128
      },
      "opponent": {
        "tag": "#2PVJC8R2G",
        "name": "❄SAKINAHLEGION❄",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WGc-x4nogFr6bd5nfGOfU6fSXeMIOGRo6rEsCkAxtbA.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WGc-x4nogFr6bd5nfGOfU6fSXeMIOGRo6rEsCkAxtbA.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WGc-x4nogFr6bd5nfGOfU6fSXeMIOGRo6rEsCkAxtbA.png"
        },
        "clanLevel": 12,
        "stars": 25,
        "destructionPercentage": 87.5
      }
    },
    {
      "result": "win",
      "endTime": "20210626T201946.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 14,
        "stars": 23,
        "destructionPercentage": 84.4,
        "expEarned": 170
      },
      "opponent": {
        "tag": "#GU9PJ2PQ",
        "name": "rockstar",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/QQTdjObiG8rIkwSfBG5eOjJS_pgZffGeiOLMDA4O2YI.png",
          "large": "https://api-assets.clashofclans.com/badges/512/QQTdjObiG8rIkwSfBG5eOjJS_pgZffGeiOLMDA4O2YI.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/QQTdjObiG8rIkwSfBG5eOjJS_pgZffGeiOLMDA4O2YI.png"
        },
        "clanLevel": 15,
        "stars": 17,
        "destructionPercentage": 69.8
      }
    },
    {
      "result": "win",
      "endTime": "20210617T005259.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 18,
        "stars": 25,
        "destructionPercentage": 90.9,
        "expEarned": 179
      },
      "opponent": {
        "tag": "#PCPJQVQU",
        "name": "北国狼族",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/wDs1c81yVuWgSmAAhh9mM8y86Mb7Q-ZuIFQIXKKyGwc.png",
          "large": "https://api-assets.clashofclans.com/badges/512/wDs1c81yVuWgSmAAhh9mM8y86Mb7Q-ZuIFQIXKKyGwc.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/wDs1c81yVuWgSmAAhh9mM8y86Mb7Q-ZuIFQIXKKyGwc.png"
        },
        "clanLevel": 13,
        "stars": 14,
        "destructionPercentage": 61.8
      }
    },
    {
      "result": "win",
      "endTime": "20210614T215336.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 19,
        "stars": 23,
        "destructionPercentage": 90.6,
        "expEarned": 177
      },
      "opponent": {
        "tag": "#88ULG9CC",
        "name": "BRUTALS RULES",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/1mIDnEotV23jKw3RE4WBukECZ-0i-9i8FhaEJRgjRWY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/1mIDnEotV23jKw3RE4WBukECZ-0i-9i8FhaEJRgjRWY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/1mIDnEotV23jKw3RE4WBukECZ-0i-9i8FhaEJRgjRWY.png"
        },
        "clanLevel": 16,
        "stars": 21,
        "destructionPercentage": 78.9
      }
    },
    {
      "result": null,
      "endTime": "20210611T032633.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 100,
        "stars": 259,
        "destructionPercentage": 531.2,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 200,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20210531T221701.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 16,
        "stars": 26,
        "destructionPercentage": 87.5,
        "expEarned": 175
      },
      "opponent": {
        "tag": "#29JPP89CC",
        "name": "Sultan Jr",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/ClNiMpVTrMXr_nr8mq-__midwbG5Y-VjjkwdQ05_Mos.png",
          "large": "https://api-assets.clashofclans.com/badges/512/ClNiMpVTrMXr_nr8mq-__midwbG5Y-VjjkwdQ05_Mos.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/ClNiMpVTrMXr_nr8mq-__midwbG5Y-VjjkwdQ05_Mos.png"
        },
        "clanLevel": 11,
        "stars": 23,
        "destructionPercentage": 87.4
      }
    },
    {
      "result": "lose",
      "endTime": "20210529T211433.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 17,
        "stars": 21,
        "destructionPercentage": 71.9,
        "expEarned": 131
      },
      "opponent": {
        "tag": "#9C2LLVGP",
        "name": "Strong Lions",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/1mIDnEotV23jKw3RE4WBukECZ-0i-9i8FhaEJRgjRWY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/1mIDnEotV23jKw3RE4WBukECZ-0i-9i8FhaEJRgjRWY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/1mIDnEotV23jKw3RE4WBukECZ-0i-9i8FhaEJRgjRWY.png"
        },
        "clanLevel": 16,
        "stars": 27,
        "destructionPercentage": 89.3
      }
    },
    {
      "result": "win",
      "endTime": "20210526T231000.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 17,
        "stars": 28,
        "destructionPercentage": 93.4,
        "expEarned": 176
      },
      "opponent": {
        "tag": "#28PQJPL2Y",
        "name": "花陌忧离殇",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/GoNtT_NyEEsrij1PADmKF6XZcyHTqdvXTCo8kDzu3EA.png",
          "large": "https://api-assets.clashofclans.com/badges/512/GoNtT_NyEEsrij1PADmKF6XZcyHTqdvXTCo8kDzu3EA.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/GoNtT_NyEEsrij1PADmKF6XZcyHTqdvXTCo8kDzu3EA.png"
        },
        "clanLevel": 10,
        "stars": 9,
        "destructionPercentage": 34.4
      }
    },
    {
      "result": "win",
      "endTime": "20210522T220203.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 18,
        "stars": 26,
        "destructionPercentage": 93.4,
        "expEarned": 174
      },
      "opponent": {
        "tag": "#Y00RLVQ",
        "name": "Aurgiclan",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/FZclOlJAddEzv6SF2tr_Z7oVLFU118tTVu0vgrGb6OE.png",
          "large": "https://api-assets.clashofclans.com/badges/512/FZclOlJAddEzv6SF2tr_Z7oVLFU118tTVu0vgrGb6OE.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/FZclOlJAddEzv6SF2tr_Z7oVLFU118tTVu0vgrGb6OE.png"
        },
        "clanLevel": 16,
        "stars": 8,
        "destructionPercentage": 38.8
      }
    },
    {
      "result": "win",
      "endTime": "20210520T204220.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 18,
        "stars": 25,
        "destructionPercentage": 84,
        "expEarned": 179
      },
      "opponent": {
        "tag": "#9J0P0LLP",
        "name": "مملكة الظلام",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/Az2KTu1Fo-2eOl6555nLAbcJvV_x3iW-TjJVM8vqt0s.png",
          "large": "https://api-assets.clashofclans.com/badges/512/Az2KTu1Fo-2eOl6555nLAbcJvV_x3iW-TjJVM8vqt0s.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/Az2KTu1Fo-2eOl6555nLAbcJvV_x3iW-TjJVM8vqt0s.png"
        },
        "clanLevel": 17,
        "stars": 23,
        "destructionPercentage": 84.8
      }
    },
    {
      "result": "lose",
      "endTime": "20210517T220027.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 17,
        "stars": 24,
        "destructionPercentage": 83.7,
        "expEarned": 121
      },
      "opponent": {
        "tag": "#PYURYG92",
        "name": "namber one",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/NXqed3x8BT1pRtQJmODCivTaEYqwknWuLBY3IHks4p4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/NXqed3x8BT1pRtQJmODCivTaEYqwknWuLBY3IHks4p4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/NXqed3x8BT1pRtQJmODCivTaEYqwknWuLBY3IHks4p4.png"
        },
        "clanLevel": 19,
        "stars": 27,
        "destructionPercentage": 90.4
      }
    },
    {
      "result": "lose",
      "endTime": "20210512T233543.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 28,
        "stars": 36,
        "destructionPercentage": 80.4,
        "expEarned": 171
      },
      "opponent": {
        "tag": "#2JJL99RR",
        "name": "台灣 - 台北指揮部 4",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/bf6Z5RX_usEpQ3CwSSUJAWvVvLLXox5htTABYy8MU_Q.png",
          "large": "https://api-assets.clashofclans.com/badges/512/bf6Z5RX_usEpQ3CwSSUJAWvVvLLXox5htTABYy8MU_Q.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/bf6Z5RX_usEpQ3CwSSUJAWvVvLLXox5htTABYy8MU_Q.png"
        },
        "clanLevel": 21,
        "stars": 40,
        "destructionPercentage": 96.333336
      }
    },
    {
      "result": "tie",
      "endTime": "20210509T215055.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 98,
        "stars": 140,
        "destructionPercentage": 379.33334,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 228,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20210429T233154.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 17,
        "stars": 22,
        "destructionPercentage": 79.6,
        "expEarned": 129
      },
      "opponent": {
        "tag": "#CC8QG8GR",
        "name": "BLACK PANTHERS✊",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/VNO3fYa9l0BfUpnf-IYqisdU2yI9vBeHQwjqLseck3c.png",
          "large": "https://api-assets.clashofclans.com/badges/512/VNO3fYa9l0BfUpnf-IYqisdU2yI9vBeHQwjqLseck3c.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/VNO3fYa9l0BfUpnf-IYqisdU2yI9vBeHQwjqLseck3c.png"
        },
        "clanLevel": 15,
        "stars": 25,
        "destructionPercentage": 82.8
      }
    },
    {
      "result": "win",
      "endTime": "20210427T222454.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 24,
        "stars": 36,
        "destructionPercentage": 91.6,
        "expEarned": 210
      },
      "opponent": {
        "tag": "#G899VVQY",
        "name": "# The New Start",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/SE4UDVVADxhRDD0mM8G2MWdvB6wPmT7jj-ea9yV2RkE.png",
          "large": "https://api-assets.clashofclans.com/badges/512/SE4UDVVADxhRDD0mM8G2MWdvB6wPmT7jj-ea9yV2RkE.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/SE4UDVVADxhRDD0mM8G2MWdvB6wPmT7jj-ea9yV2RkE.png"
        },
        "clanLevel": 14,
        "stars": 32,
        "destructionPercentage": 79.46667
      }
    },
    {
      "result": "lose",
      "endTime": "20210425T032543.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 18,
        "stars": 22,
        "destructionPercentage": 80.6,
        "expEarned": 124
      },
      "opponent": {
        "tag": "#CP0LGRUL",
        "name": "حلفايا♡",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/RqXOXEZkzC56979qYcMGVtUle7G0Yu0DaOLKoJQvEVw.png",
          "large": "https://api-assets.clashofclans.com/badges/512/RqXOXEZkzC56979qYcMGVtUle7G0Yu0DaOLKoJQvEVw.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/RqXOXEZkzC56979qYcMGVtUle7G0Yu0DaOLKoJQvEVw.png"
        },
        "clanLevel": 13,
        "stars": 26,
        "destructionPercentage": 92.7
      }
    },
    {
      "result": "win",
      "endTime": "20210421T220919.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 20,
        "stars": 28,
        "destructionPercentage": 95,
        "expEarned": 175
      },
      "opponent": {
        "tag": "#QP9ULCCG",
        "name": "Relax",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/lR-pmaHw6qARhiUtkWiXAqsiXSwuYk7eV6u_JaLGm68.png",
          "large": "https://api-assets.clashofclans.com/badges/512/lR-pmaHw6qARhiUtkWiXAqsiXSwuYk7eV6u_JaLGm68.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/lR-pmaHw6qARhiUtkWiXAqsiXSwuYk7eV6u_JaLGm68.png"
        },
        "clanLevel": 19,
        "stars": 19,
        "destructionPercentage": 74
      }
    },
    {
      "result": "lose",
      "endTime": "20210419T220131.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 16,
        "stars": 23,
        "destructionPercentage": 89.5,
        "expEarned": 119
      },
      "opponent": {
        "tag": "#29CL2YG00",
        "name": "KOROKLOKOS SG",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/CbjGgcd419Rb141icrn5hmZibtZ83pEMe94AXm5mZfs.png",
          "large": "https://api-assets.clashofclans.com/badges/512/CbjGgcd419Rb141icrn5hmZibtZ83pEMe94AXm5mZfs.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/CbjGgcd419Rb141icrn5hmZibtZ83pEMe94AXm5mZfs.png"
        },
        "clanLevel": 6,
        "stars": 27,
        "destructionPercentage": 89.5
      }
    },
    {
      "result": "lose",
      "endTime": "20210417T222904.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 16,
        "stars": 26,
        "destructionPercentage": 86.1,
        "expEarned": 120
      },
      "opponent": {
        "tag": "#2LPQPRJC2",
        "name": "ReD StadiuM",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/kE1nXIvMur8r-5Gf59jDkC0-zUQezN_heHigyQrO4s0.png",
          "large": "https://api-assets.clashofclans.com/badges/512/kE1nXIvMur8r-5Gf59jDkC0-zUQezN_heHigyQrO4s0.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/kE1nXIvMur8r-5Gf59jDkC0-zUQezN_heHigyQrO4s0.png"
        },
        "clanLevel": 3,
        "stars": 26,
        "destructionPercentage": 91.3
      }
    },
    {
      "result": "win",
      "endTime": "20210415T222517.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 19,
        "stars": 27,
        "destructionPercentage": 92.3,
        "expEarned": 167
      },
      "opponent": {
        "tag": "#QPVYG980",
        "name": "ОРЛЫ ДАГЕСТАНА",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/cYpcvm2Nsm3BjDMlhrFj0TVvjwxuEisgV2IG1Sd1axw.png",
          "large": "https://api-assets.clashofclans.com/badges/512/cYpcvm2Nsm3BjDMlhrFj0TVvjwxuEisgV2IG1Sd1axw.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/cYpcvm2Nsm3BjDMlhrFj0TVvjwxuEisgV2IG1Sd1axw.png"
        },
        "clanLevel": 17,
        "stars": 20,
        "destructionPercentage": 70.5
      }
    },
    {
      "result": "lose",
      "endTime": "20210413T005832.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 22,
        "stars": 36,
        "destructionPercentage": 88,
        "expEarned": 145
      },
      "opponent": {
        "tag": "#2J20P82L",
        "name": "izmirr",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/u6kYrwr4YQ5h8okUmT00naGbeRvCnfLrM_3GbCws3f4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/u6kYrwr4YQ5h8okUmT00naGbeRvCnfLrM_3GbCws3f4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/u6kYrwr4YQ5h8okUmT00naGbeRvCnfLrM_3GbCws3f4.png"
        },
        "clanLevel": 14,
        "stars": 41,
        "destructionPercentage": 94.8
      }
    },
    {
      "result": null,
      "endTime": "20210410T010803.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 104,
        "stars": 173,
        "destructionPercentage": 404.26666,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 217,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20210401T224311.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 19,
        "stars": 28,
        "destructionPercentage": 92,
        "expEarned": 166
      },
      "opponent": {
        "tag": "#PL8UCYL2",
        "name": "aligarians.....",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/qr4R_Wd62N9Wp6jWao2e16JjYO_82j7xMF8oolXc5mk.png",
          "large": "https://api-assets.clashofclans.com/badges/512/qr4R_Wd62N9Wp6jWao2e16JjYO_82j7xMF8oolXc5mk.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/qr4R_Wd62N9Wp6jWao2e16JjYO_82j7xMF8oolXc5mk.png"
        },
        "clanLevel": 16,
        "stars": 26,
        "destructionPercentage": 94.8
      }
    },
    {
      "result": "lose",
      "endTime": "20210330T043140.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 17,
        "stars": 27,
        "destructionPercentage": 93,
        "expEarned": 121
      },
      "opponent": {
        "tag": "#92PL8YGG",
        "name": "Spartaa",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/bJ5SBEac1IO9idyflhzzyLi71QgUodksRzOt9BmrMT4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/bJ5SBEac1IO9idyflhzzyLi71QgUodksRzOt9BmrMT4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/bJ5SBEac1IO9idyflhzzyLi71QgUodksRzOt9BmrMT4.png"
        },
        "clanLevel": 14,
        "stars": 27,
        "destructionPercentage": 93.6
      }
    },
    {
      "result": "win",
      "endTime": "20210328T003329.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 16,
        "stars": 27,
        "destructionPercentage": 91.7,
        "expEarned": 172
      },
      "opponent": {
        "tag": "#28R9RVQP0",
        "name": "EXiLaDoS [019]",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/dCYB3bdhaEJ2vxjBURtxoFj4jshWC5baeKBSQytthxU.png",
          "large": "https://api-assets.clashofclans.com/badges/512/dCYB3bdhaEJ2vxjBURtxoFj4jshWC5baeKBSQytthxU.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/dCYB3bdhaEJ2vxjBURtxoFj4jshWC5baeKBSQytthxU.png"
        },
        "clanLevel": 9,
        "stars": 16,
        "destructionPercentage": 68.4
      }
    },
    {
      "result": "lose",
      "endTime": "20210326T000846.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 23,
        "stars": 37,
        "destructionPercentage": 85.2,
        "expEarned": 140
      },
      "opponent": {
        "tag": "#PPRJGCJ8",
        "name": "DiamondDevils",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/D1n3GaNYPnvDeqW-347fhCrwiCvdJh9rOL7jXbYcCZo.png",
          "large": "https://api-assets.clashofclans.com/badges/512/D1n3GaNYPnvDeqW-347fhCrwiCvdJh9rOL7jXbYcCZo.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/D1n3GaNYPnvDeqW-347fhCrwiCvdJh9rOL7jXbYcCZo.png"
        },
        "clanLevel": 14,
        "stars": 37,
        "destructionPercentage": 89.2
      }
    },
    {
      "result": "win",
      "endTime": "20210324T003042.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 19,
        "stars": 28,
        "destructionPercentage": 92.2,
        "expEarned": 169
      },
      "opponent": {
        "tag": "#899YP0VJ",
        "name": "KARAMBOL SQUAD",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/yzEGPFP5T5XQ_GNFE993EMjGEJHFgE0lYrkNvWkGnrw.png",
          "large": "https://api-assets.clashofclans.com/badges/512/yzEGPFP5T5XQ_GNFE993EMjGEJHFgE0lYrkNvWkGnrw.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/yzEGPFP5T5XQ_GNFE993EMjGEJHFgE0lYrkNvWkGnrw.png"
        },
        "clanLevel": 15,
        "stars": 27,
        "destructionPercentage": 95.3
      }
    },
    {
      "result": "lose",
      "endTime": "20210322T005746.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 22,
        "stars": 30,
        "destructionPercentage": 76.066666,
        "expEarned": 150
      },
      "opponent": {
        "tag": "#29CLYCRJ9",
        "name": "Šup³3r ₩♡Ŕ",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/XcKYX073fdjh2Uhx5PTWkcnI0wTY19modXJDn1flD2Y.png",
          "large": "https://api-assets.clashofclans.com/badges/512/XcKYX073fdjh2Uhx5PTWkcnI0wTY19modXJDn1flD2Y.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/XcKYX073fdjh2Uhx5PTWkcnI0wTY19modXJDn1flD2Y.png"
        },
        "clanLevel": 10,
        "stars": 35,
        "destructionPercentage": 82.26667
      }
    },
    {
      "result": "lose",
      "endTime": "20210319T235021.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 20,
        "stars": 36,
        "destructionPercentage": 81,
        "expEarned": 160
      },
      "opponent": {
        "tag": "#29C8299V8",
        "name": "AA狼群AA",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/DmpBfZByMSV2lH0pQ7fN465XLor5R20-zeaf0t4zc78.png",
          "large": "https://api-assets.clashofclans.com/badges/512/DmpBfZByMSV2lH0pQ7fN465XLor5R20-zeaf0t4zc78.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/DmpBfZByMSV2lH0pQ7fN465XLor5R20-zeaf0t4zc78.png"
        },
        "clanLevel": 11,
        "stars": 37,
        "destructionPercentage": 86.8
      }
    },
    {
      "result": "win",
      "endTime": "20210318T001751.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 26,
        "stars": 37,
        "destructionPercentage": 86.666664,
        "expEarned": 209
      },
      "opponent": {
        "tag": "#J9J228PG",
        "name": "SLEEPY ZONE",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/WchZ_zz5TTbt7yt8HOFW4hp69qSI28SU4zxc9qNzTv4.png",
          "large": "https://api-assets.clashofclans.com/badges/512/WchZ_zz5TTbt7yt8HOFW4hp69qSI28SU4zxc9qNzTv4.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/WchZ_zz5TTbt7yt8HOFW4hp69qSI28SU4zxc9qNzTv4.png"
        },
        "clanLevel": 14,
        "stars": 31,
        "destructionPercentage": 73.666664
      }
    },
    {
      "result": "lose",
      "endTime": "20210315T230654.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 24,
        "stars": 38,
        "destructionPercentage": 88.666664,
        "expEarned": 154
      },
      "opponent": {
        "tag": "#2PRQLL8GR",
        "name": "1 Disabled Army",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/1-6ys9R6mB_9hjW3J9q3sMDQuUCJplDlwF91CI_8eAw.png",
          "large": "https://api-assets.clashofclans.com/badges/512/1-6ys9R6mB_9hjW3J9q3sMDQuUCJplDlwF91CI_8eAw.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/1-6ys9R6mB_9hjW3J9q3sMDQuUCJplDlwF91CI_8eAw.png"
        },
        "clanLevel": 9,
        "stars": 42,
        "destructionPercentage": 97.666664
      }
    },
    {
      "result": "win",
      "endTime": "20210313T224503.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 27,
        "stars": 37,
        "destructionPercentage": 90.13333,
        "expEarned": 214
      },
      "opponent": {
        "tag": "#UJJGYJ88",
        "name": "SAN RAFAEL PY",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/lh5138F8tqr755o4PEhvhwnDuJTw4x_eEYCbV9ZU56E.png",
          "large": "https://api-assets.clashofclans.com/badges/512/lh5138F8tqr755o4PEhvhwnDuJTw4x_eEYCbV9ZU56E.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/lh5138F8tqr755o4PEhvhwnDuJTw4x_eEYCbV9ZU56E.png"
        },
        "clanLevel": 14,
        "stars": 35,
        "destructionPercentage": 83
      }
    },
    {
      "result": "win",
      "endTime": "20210311T031213.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 103,
        "stars": 267,
        "destructionPercentage": 508.86667,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 131,
        "destructionPercentage": 0
      }
    },
    {
      "result": "lose",
      "endTime": "20210228T013357.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 20,
        "stars": 36,
        "destructionPercentage": 91.333336,
        "expEarned": 158
      },
      "opponent": {
        "tag": "#92Q9GRJ2",
        "name": "yod'z wAr herO",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/1WbO1PjlMTE3BtByprn7pV_hh4VXppW4NqebLcExnOw.png",
          "large": "https://api-assets.clashofclans.com/badges/512/1WbO1PjlMTE3BtByprn7pV_hh4VXppW4NqebLcExnOw.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/1WbO1PjlMTE3BtByprn7pV_hh4VXppW4NqebLcExnOw.png"
        },
        "clanLevel": 16,
        "stars": 37,
        "destructionPercentage": 88.8
      }
    },
    {
      "result": "win",
      "endTime": "20210226T002659.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 23,
        "stars": 36,
        "destructionPercentage": 88.46667,
        "expEarned": 214
      },
      "opponent": {
        "tag": "#2PP092LJC",
        "name": "AMX",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/hwxb3soejDVYdJZxioTqd8Lbc3LxkQ7m8VgME9xFwAc.png",
          "large": "https://api-assets.clashofclans.com/badges/512/hwxb3soejDVYdJZxioTqd8Lbc3LxkQ7m8VgME9xFwAc.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/hwxb3soejDVYdJZxioTqd8Lbc3LxkQ7m8VgME9xFwAc.png"
        },
        "clanLevel": 6,
        "stars": 30,
        "destructionPercentage": 80.066666
      }
    },
    {
      "result": "win",
      "endTime": "20210224T001120.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 24,
        "stars": 37,
        "destructionPercentage": 89,
        "expEarned": 203
      },
      "opponent": {
        "tag": "#UQGV09Q",
        "name": "Oricy Fed",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/BjzoDWcRwjTnV6ly8Q1YR1an__8RFAUAPcdZxJykHuA.png",
          "large": "https://api-assets.clashofclans.com/badges/512/BjzoDWcRwjTnV6ly8Q1YR1an__8RFAUAPcdZxJykHuA.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/BjzoDWcRwjTnV6ly8Q1YR1an__8RFAUAPcdZxJykHuA.png"
        },
        "clanLevel": 18,
        "stars": 29,
        "destructionPercentage": 76
      }
    },
    {
      "result": "lose",
      "endTime": "20210221T213902.000Z",
      "teamSize": 15,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 24,
        "stars": 38,
        "destructionPercentage": 88.2,
        "expEarned": 141
      },
      "opponent": {
        "tag": "#8CQYGCUU",
        "name": "Smarty",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/Lsn_hrbbHQ-NaBdNXy-rjCzuaNj42XhpnuZllWDy3-I.png",
          "large": "https://api-assets.clashofclans.com/badges/512/Lsn_hrbbHQ-NaBdNXy-rjCzuaNj42XhpnuZllWDy3-I.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/Lsn_hrbbHQ-NaBdNXy-rjCzuaNj42XhpnuZllWDy3-I.png"
        },
        "clanLevel": 17,
        "stars": 41,
        "destructionPercentage": 96.933334
      }
    },
    {
      "result": "win",
      "endTime": "20210218T233700.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 18,
        "stars": 28,
        "destructionPercentage": 95.8,
        "expEarned": 163
      },
      "opponent": {
        "tag": "#P0RL0UCJ",
        "name": "anime lovers",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/6Ed2aYCf8JzHn9czg-lfwt8cRXXGzxLbRS0dAPcgqPA.png",
          "large": "https://api-assets.clashofclans.com/badges/512/6Ed2aYCf8JzHn9czg-lfwt8cRXXGzxLbRS0dAPcgqPA.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/6Ed2aYCf8JzHn9czg-lfwt8cRXXGzxLbRS0dAPcgqPA.png"
        },
        "clanLevel": 18,
        "stars": 28,
        "destructionPercentage": 94.4
      }
    },
    {
      "result": "win",
      "endTime": "20210217T000122.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 18,
        "stars": 28,
        "destructionPercentage": 93.5,
        "expEarned": 159
      },
      "opponent": {
        "tag": "#P2PYR89Y",
        "name": "Dark dragon afg",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/otvLj5AFX0DDoTRXpTVtrkngN3y_my_V-Sx9CWm0SRM.png",
          "large": "https://api-assets.clashofclans.com/badges/512/otvLj5AFX0DDoTRXpTVtrkngN3y_my_V-Sx9CWm0SRM.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/otvLj5AFX0DDoTRXpTVtrkngN3y_my_V-Sx9CWm0SRM.png"
        },
        "clanLevel": 16,
        "stars": 14,
        "destructionPercentage": 62.5
      }
    },
    {
      "result": "lose",
      "endTime": "20210212T184846.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 15,
        "stars": 21,
        "destructionPercentage": 82.1,
        "expEarned": 116
      },
      "opponent": {
        "tag": "#8G0ULQVP",
        "name": "عراقي للابد",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/Hl6Aj9EdL_nNBZLSL4pXNwV0lzQbGvEUVgb7WH3EGwY.png",
          "large": "https://api-assets.clashofclans.com/badges/512/Hl6Aj9EdL_nNBZLSL4pXNwV0lzQbGvEUVgb7WH3EGwY.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/Hl6Aj9EdL_nNBZLSL4pXNwV0lzQbGvEUVgb7WH3EGwY.png"
        },
        "clanLevel": 12,
        "stars": 26,
        "destructionPercentage": 87.3
      }
    },
    {
      "result": "win",
      "endTime": "20210210T005109.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 104,
        "stars": 276,
        "destructionPercentage": 509.4,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 162,
        "destructionPercentage": 0
      }
    },
    {
      "result": "win",
      "endTime": "20210201T235601.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 18,
        "stars": 26,
        "destructionPercentage": 94.2,
        "expEarned": 172
      },
      "opponent": {
        "tag": "#22UQRGRP9",
        "name": "kogyi",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/HPKE1lha58e3GmKn4CK5r9SK-WYkVSMysUTDduB7WDk.png",
          "large": "https://api-assets.clashofclans.com/badges/512/HPKE1lha58e3GmKn4CK5r9SK-WYkVSMysUTDduB7WDk.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/HPKE1lha58e3GmKn4CK5r9SK-WYkVSMysUTDduB7WDk.png"
        },
        "clanLevel": 9,
        "stars": 5,
        "destructionPercentage": 18
      }
    },
    {
      "result": "lose",
      "endTime": "20210130T231733.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 18,
        "stars": 25,
        "destructionPercentage": 90,
        "expEarned": 109
      },
      "opponent": {
        "tag": "#YPU9PJY",
        "name": "BD Death Nights",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/AvHMKxinMJH2H_MSwCB_Z6IW4klgOFCVLvXwx4A2LdA.png",
          "large": "https://api-assets.clashofclans.com/badges/512/AvHMKxinMJH2H_MSwCB_Z6IW4klgOFCVLvXwx4A2LdA.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/AvHMKxinMJH2H_MSwCB_Z6IW4klgOFCVLvXwx4A2LdA.png"
        },
        "clanLevel": 16,
        "stars": 28,
        "destructionPercentage": 93.6
      }
    },
    {
      "result": "win",
      "endTime": "20210128T002847.000Z",
      "teamSize": 10,
      "attacksPerMember": 2,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 16,
        "stars": 26,
        "destructionPercentage": 91.8,
        "expEarned": 174
      },
      "opponent": {
        "tag": "#2998CGGU8",
        "name": "L.B.tribe 2.0",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/26mWzOqonO2Bxai6JIG18KKknq9IU1JWFcOy2608J44.png",
          "large": "https://api-assets.clashofclans.com/badges/512/26mWzOqonO2Bxai6JIG18KKknq9IU1JWFcOy2608J44.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/26mWzOqonO2Bxai6JIG18KKknq9IU1JWFcOy2608J44.png"
        },
        "clanLevel": 8,
        "stars": 17,
        "destructionPercentage": 65.1
      }
    },
    {
      "result": null,
      "endTime": "20210109T234513.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "large": "https://api-assets.clashofclans.com/badges/512/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/n9DoGyYRh7Tw-CudYNEQ9lLjWsL7WMVeiY26NGpOdng.png"
        },
        "clanLevel": 18,
        "attacks": 89,
        "stars": 230,
        "destructionPercentage": 443.26666,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 171,
        "destructionPercentage": 0
      }
    },
    {
      "result": null,
      "endTime": "20201210T024528.000Z",
      "teamSize": 15,
      "attacksPerMember": 1,
      "clan": {
        "tag": "#209QRU22",
        "name": "Израиль 30+",
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/68InGXD0yxY7OxqPHX1EoPn7dJqjIHS_u9zDfcwkG5o.png",
          "large": "https://api-assets.clashofclans.com/badges/512/68InGXD0yxY7OxqPHX1EoPn7dJqjIHS_u9zDfcwkG5o.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/68InGXD0yxY7OxqPHX1EoPn7dJqjIHS_u9zDfcwkG5o.png"
        },
        "clanLevel": 17,
        "attacks": 98,
        "stars": 197,
        "destructionPercentage": 427.8,
        "expEarned": 0
      },
      "opponent": {
        "badgeUrls": {
          "small": "https://api-assets.clashofclans.com/badges/70/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "large": "https://api-assets.clashofclans.com/badges/512/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png",
          "medium": "https://api-assets.clashofclans.com/badges/200/H39b_-WLZGtZVWQ0hqTkE-Tn2AaQnQWy_Iz4yBlvL0M.png"
        },
        "clanLevel": 0,
        "stars": 180,
        "destructionPercentage": 0
      }
    }
  ],
  "paging": {
    "cursors": {}
  }
}
"##;

#[test]
fn warlog() {
    let warlog: ClanWarLog = json::from_str(WARLOG_JSON).unwrap();
    assert_eq!(warlog.items.len(), 119);
}

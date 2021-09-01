# Train report for javascript / file:///tmp/top-repos-quality-repos-pl1imr1i/hair-shop-cap.git HEAD c43ea6866b5e7f76895a194f42c8a1d549cc04bf

### Classification report

PPCR: 0.649

| Class | Precision | Recall | Full Recall | F1-score | Full F1-score | Support | Full Support | PPCR |
|------:|:----------|:-------|:------------|:---------|:---------|:--------|:-------------|:-----|
| `∅` | 0.939| 1.000| 0.879| 0.969| 0.908| 9577| 10901| 0.879 |
| `"` | 1.000| 1.000| 0.500| 1.000| 0.667| 587| 1174| 0.500 |
| `␣` | 0.000| 0.000| 0.000| 0.000| 0.000| 255| 2677| 0.095 |
| `⏎␣⁻␣⁻` | 0.000| 0.000| 0.000| 0.000| 0.000| 255| 492| 0.518 |
| `⏎` | 0.000| 0.000| 0.000| 0.000| 0.000| 59| 868| 0.068 |
| `⏎␣⁺␣⁺` | 0.000| 0.000| 0.000| 0.000| 0.000| 53| 495| 0.107 |
| `micro avg` | 0.942| 0.942| 0.612| 0.942| 0.742| 10786| 16607| 0.649 |
| `weighted avg` | 0.888| 0.942| 0.612| 0.914| 0.643| 10786| 16607| 0.649 |
| `macro avg` | 0.323| 0.333| 0.230| 0.328| 0.262| 10786| 16607| 0.649 |

### Confusion matrix

|refusal|  ∅| ␣| "| ⏎| ⏎␣⁺␣⁺| ⏎␣⁻␣⁻| 
|:---|:---|:---|:---|:---|:---|:---|
|0 |0 |0 |0 |0 |0 |0 |
|1324 |9577 |0 |0 |0 |0 |0 |
|2422 |255 |0 |0 |0 |0 |0 |
|587 |0 |0 |587 |0 |0 |0 |
|809 |59 |0 |0 |0 |0 |0 |
|442 |53 |0 |0 |0 |0 |0 |
|237 |255 |0 |0 |0 |0 |0 |

### Files with most errors

| filename | number of errors|
|:----:|:-----|
| src/components/applicationViews/ApplicationViews.js | 214 |
| src/components/user/requestAppointment/EditModule.js | 47 |
| src/components/cards/RequestCard.js | 42 |
| src/components/admin/appointments/Appointments.js | 34 |
| src/components/cards/AppointmentCard.js | 32 |
| src/components/user/requestAppointment/RequestAppointment.js | 26 |
| src/components/admin/users/UserDetailModal.js | 17 |
| src/components/HairShop.js | 16 |
| src/components/user/profile/Profile.js | 16 |
| src/components/navBar/userNavBar/UserNav.js | 16 |

<details>
    <summary>Machine-readable report</summary>
```json
{
  "cl_report": {"\"": {"f1-score": 1.0, "precision": 1.0, "recall": 1.0, "support": 587}, "macro avg": {"f1-score": 0.32809128910463864, "precision": 0.32316893813118935, "recall": 0.3333333333333333, "support": 10786}, "micro avg": {"f1-score": 0.942332653439644, "precision": 0.942332653439644, "recall": 0.942332653439644, "support": 10786}, "weighted avg": {"f1-score": 0.9144058645031286, "precision": 0.8881822290834788, "recall": 0.942332653439644, "support": 10786}, "\u2205": {"f1-score": 0.9685477346278317, "precision": 0.939013628787136, "recall": 1.0, "support": 9577}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 59}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 53}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 255}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 255}},
  "cl_report_full": {"\"": {"f1-score": 0.6666666666666666, "precision": 1.0, "recall": 0.5, "support": 1174}, "macro avg": {"f1-score": 0.26240652975250134, "precision": 0.32316893813118935, "recall": 0.2297572088187628, "support": 16607}, "micro avg": {"f1-score": 0.7420873945898587, "precision": 0.942332653439644, "recall": 0.6120310712350214, "support": 16607}, "weighted avg": {"f1-score": 0.6429996277669316, "precision": 0.6870709681103492, "recall": 0.6120310712350214, "support": 16607}, "\u2205": {"f1-score": 0.9077725118483413, "precision": 0.939013628787136, "recall": 0.8785432529125768, "support": 10901}, "\u23ce": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 868}, "\u23ce\u2423\u207a\u2423\u207a": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 495}, "\u23ce\u2423\u207b\u2423\u207b": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 492}, "\u2423": {"f1-score": 0.0, "precision": 0.0, "recall": 0.0, "support": 2677}},
  "ppcr": 0.6494851568615644
}
```
</details>

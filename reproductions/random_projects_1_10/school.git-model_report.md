# Model report for file:///tmp/top-repos-quality-repos-mmsrg2h3/school.git HEAD fd39f8a886379088a949b7d42bdea251e01de412

### Dump

```json
{'created_at': '2021-08-22 12:28:12',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.3 kB',
 'tags': [],
 'uuid': '836c9edb-36a9-4be4-af8d-526bf84718ce',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mmsrg2h3/school.git fd39f8a886379088a949b7d42bdea251e01de412

# javascript
26 rules, avg.len. 7.2
## train
PPCR: 0.983667
### report
macro
{'f1-score': 0.8235473376909015,
 'precision': 0.8343032807113456,
 'recall': 0.81461054493856,
 'support': 56671}
micro
{'f1-score': 0.972172716204055,
 'precision': 0.972172716204055,
 'recall': 0.972172716204055,
 'support': 56671}
weighted
{'f1-score': 0.9708959184839135,
 'precision': 0.9704824769825063,
 'recall': 0.972172716204055,
 'support': 56671}
### report_full
macro
{'f1-score': 0.8017665247069251,
 'precision': 0.8343032807113456,
 'recall': 0.7780478744058072,
 'support': 57612}
micro
{'f1-score': 0.964167898987601,
 'precision': 0.972172716204055,
 'recall': 0.95629382767479,
 'support': 57612}
weighted
{'f1-score': 0.9620669881345559,
 'precision': 0.9699034320838651,
 'recall': 0.95629382767479,
 'support': 57612}
## test
PPCR: 0.983512
### report
macro
{'f1-score': 0.8294436286900402,
 'precision': 0.8463143721998455,
 'recall': 0.8145993792526679,
 'support': 41933}
micro
{'f1-score': 0.9838790451434432,
 'precision': 0.9838790451434432,
 'recall': 0.9838790451434432,
 'support': 41933}
weighted
{'f1-score': 0.9838317172323667,
 'precision': 0.984234136097195,
 'recall': 0.9838790451434432,
 'support': 41933}
### report_full
macro
{'f1-score': 0.8035248303947203,
 'precision': 0.8463143721998455,
 'recall': 0.7727361302769561,
 'support': 42636}
micro
{'f1-score': 0.9757003157185259,
 'precision': 0.9838790451434432,
 'recall': 0.9676564405666573,
 'support': 42636}
weighted
{'f1-score': 0.9749182355786403,
 'precision': 0.9842268436198975,
 'recall': 0.9676564405666573,
 'support': 42636}
```

## javascript
### Summary
22 rules, avg.len. 6.4

| | |
|-|-|
|Min support|109|
|Max support|12976|
|Min confidence|0.9250931739807129|
|Max confidence|0.9997703433036804|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 91,
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 12976.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 526.` |
| 3 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 222.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.979. Support: 165.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 134.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 6257.` |
| 7 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.994. Support: 1068.` |
| 8 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 2336.` |
| 9 | `  -1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.971. Support: 1263.` |
| 10 | `  -1.reserved not in {;, {}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 1622.` |
| 11 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 529.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 6240.` |
| 13 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 131.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2177.` |
| 15 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2047.` |
| 16 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 741.` |
| 17 | `  -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 312.` |
| 18 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentBlock}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.965. Support: 694.` |
| 19 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentBlock}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 109.` |
| 20 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 260.` |
| 21 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.987. Support: 197.` |
| 22 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 4292.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.409090909090909, "max_conf": 0.9997703433036804, "max_support": 12976, "min_conf": 0.9250931739807129, "min_support": 109, "num_rules": 22}}
```
</details>

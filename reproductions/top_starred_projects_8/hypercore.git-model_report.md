# Model report for file:///tmp/top-repos-quality-repos-ch9n34ot/hypercore.git HEAD becc8ce8e2317d84e60724ebaf17fdc5db3c40f4

### Dump

```json
{'created_at': '2021-08-29 16:25:21',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.6 kB',
 'tags': [],
 'uuid': 'e7207868-6a0f-4adf-9f04-ddbfc503919f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ch9n34ot/hypercore.git becc8ce8e2317d84e60724ebaf17fdc5db3c40f4

# javascript
25 rules, avg.len. 6.8
## train
PPCR: 0.951443
### report
macro
{'f1-score': 0.913070090722125,
 'precision': 0.9599096778048251,
 'recall': 0.8880586619346669,
 'support': 46184}
micro
{'f1-score': 0.9784773947687511,
 'precision': 0.9784773947687511,
 'recall': 0.9784773947687511,
 'support': 46184}
weighted
{'f1-score': 0.9776143268547467,
 'precision': 0.9786839156296372,
 'recall': 0.9784773947687511,
 'support': 46184}
### report_full
macro
{'f1-score': 0.8319185655361986,
 'precision': 0.9599096778048251,
 'recall': 0.7888119992783614,
 'support': 48541}
micro
{'f1-score': 0.9541303774082871,
 'precision': 0.9784773947687511,
 'recall': 0.9309655754928823,
 'support': 48541}
weighted
{'f1-score': 0.9446281243130032,
 'precision': 0.9758632108716433,
 'recall': 0.9309655754928823,
 'support': 48541}
## test
PPCR: 0.959571
### report
macro
{'f1-score': 0.8098887015872498,
 'precision': 0.8763357940997739,
 'recall': 0.8073402898315345,
 'support': 11820}
micro
{'f1-score': 0.9517766497461929,
 'precision': 0.9517766497461929,
 'recall': 0.9517766497461929,
 'support': 11820}
weighted
{'f1-score': 0.9533064847962289,
 'precision': 0.9605843756486533,
 'recall': 0.9517766497461929,
 'support': 11820}
### report_full
macro
{'f1-score': 0.7252493175033464,
 'precision': 0.8763357940997739,
 'recall': 0.722462996228907,
 'support': 12318}
micro
{'f1-score': 0.9321401938851603,
 'precision': 0.9517766497461929,
 'recall': 0.913297613248904,
 'support': 12318}
weighted
{'f1-score': 0.9207379610446083,
 'precision': 0.9542141029305936,
 'recall': 0.913297613248904,
 'support': 12318}
```

## javascript
### Summary
19 rules, avg.len. 6.4

| | |
|-|-|
|Min support|119|
|Max support|8116|
|Min confidence|0.9558280110359192|
|Max confidence|0.9998674988746643|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 183,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 7743.` |
| 2 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1159.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 942.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 542.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 150.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, NAME}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 119.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 7729.` |
| 8 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 401.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 3774.` |
| 10 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 872.` |
| 11 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.978. Support: 795.` |
| 12 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 785.` |
| 13 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.977. Support: 821.` |
| 14 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 425.` |
| 15 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 258.` |
| 16 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 126.` |
| 17 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.length ≥ 11<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 120.` |
| 18 | `  •••start_col ≥ 4<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 8116.` |
| 19 | `  •••start_col ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ +5.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.970. Support: 215.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.421052631578948, "max_conf": 0.9998674988746643, "max_support": 8116, "min_conf": 0.9558280110359192, "min_support": 119, "num_rules": 19}}
```
</details>

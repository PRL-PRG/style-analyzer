# Model report for file:///tmp/top-repos-quality-repos-awewez11/celluloid.git HEAD f54bfe86c60324c7f57667cb494bdd1cbb4e17c8

### Dump

```json
{'created_at': '2021-08-20 11:44:32',
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
 'size': '16.2 kB',
 'tags': [],
 'uuid': '39a20de1-6162-4998-9067-d8ee2b598bb7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-awewez11/celluloid.git f54bfe86c60324c7f57667cb494bdd1cbb4e17c8

# javascript
38 rules, avg.len. 6.6
## train
PPCR: 0.974359
### report
macro
{'f1-score': 0.7777043563276417,
 'precision': 0.805197207703052,
 'recall': 0.7562801182842332,
 'support': 12882}
micro
{'f1-score': 0.9539667753454433,
 'precision': 0.9539667753454433,
 'recall': 0.9539667753454433,
 'support': 12882}
weighted
{'f1-score': 0.9513242910611659,
 'precision': 0.951136633962426,
 'recall': 0.9539667753454433,
 'support': 12882}
### report_full
macro
{'f1-score': 0.7668489491339504,
 'precision': 0.805197207703052,
 'recall': 0.7365437364333655,
 'support': 13221}
micro
{'f1-score': 0.9415775964448531,
 'precision': 0.9539667753454433,
 'recall': 0.9295060887981242,
 'support': 13221}
weighted
{'f1-score': 0.935207545583901,
 'precision': 0.9450071314251777,
 'recall': 0.9295060887981242,
 'support': 13221}
## test
PPCR: 0.972527
### report
macro
{'f1-score': 0.7831267053924389,
 'precision': 0.8135822510822511,
 'recall': 0.7687499999999999,
 'support': 177}
micro
{'f1-score': 0.9661016949152542,
 'precision': 0.9661016949152542,
 'recall': 0.9661016949152542,
 'support': 177}
weighted
{'f1-score': 0.9650108867144812,
 'precision': 0.9682916738001485,
 'recall': 0.9661016949152542,
 'support': 177}
### report_full
macro
{'f1-score': 0.7702236616612852,
 'precision': 0.8135822510822511,
 'recall': 0.7437641723356008,
 'support': 182}
micro
{'f1-score': 0.9526462395543176,
 'precision': 0.9661016949152542,
 'recall': 0.9395604395604396,
 'support': 182}
weighted
{'f1-score': 0.9478638030008176,
 'precision': 0.962447274947275,
 'recall': 0.9395604395604396,
 'support': 182}
```

## javascript
### Summary
26 rules, avg.len. 7.3

| | |
|-|-|
|Min support|188|
|Max support|5586|
|Min confidence|0.9311023354530334|
|Max confidence|0.9992897510528564|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 110,
                     'min_samples_split': 194,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.999. Support: 704.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 515.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 440.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.955. Support: 188.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, >, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 4883.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, >}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.954. Support: 227.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 210.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 1120.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, >}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.938. Support: 264.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 223.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 598.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 5497.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 400.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.931. Support: 254.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 240.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 589.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {CALLEE}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 5558.` |
| 18 | `  -1.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 0.999. Support: 639.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 555.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, >, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {CALLEE}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 5586.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 404.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1208.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.976. Support: 4914.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 606.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 5512.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 586.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.269230769230769, "max_conf": 0.9992897510528564, "max_support": 5586, "min_conf": 0.9311023354530334, "min_support": 188, "num_rules": 26}}
```
</details>

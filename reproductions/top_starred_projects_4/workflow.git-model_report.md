# Model report for file:///tmp/top-repos-quality-repos-56o0yir_/workflow.git HEAD 59e818e3c3db3b359272d8dad05d09bd6e355266

### Dump

```json
{'created_at': '2021-08-30 08:48:11',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': 'a4552e93-c107-47cf-8e15-6a991eb7d9b9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-56o0yir_/workflow.git 59e818e3c3db3b359272d8dad05d09bd6e355266

# javascript
37 rules, avg.len. 10.6
## train
PPCR: 0.899232
### report
macro
{'f1-score': 0.9108460455644342,
 'precision': 0.9302473304776477,
 'recall': 0.8947031595895332,
 'support': 30225}
micro
{'f1-score': 0.9575186104218363,
 'precision': 0.9575186104218363,
 'recall': 0.9575186104218363,
 'support': 30225}
weighted
{'f1-score': 0.9569750035746354,
 'precision': 0.9571533140129828,
 'recall': 0.9575186104218363,
 'support': 30225}
### report_full
macro
{'f1-score': 0.7893403720086521,
 'precision': 0.9302473304776477,
 'recall': 0.7304828936604169,
 'support': 33612}
micro
{'f1-score': 0.9067155411438509,
 'precision': 0.9575186104218363,
 'recall': 0.8610317743662977,
 'support': 33612}
weighted
{'f1-score': 0.8938307694352126,
 'precision': 0.9536253239209564,
 'recall': 0.8610317743662977,
 'support': 33612}
## test
PPCR: 0.902502
### report
macro
{'f1-score': 0.8998980600286872,
 'precision': 0.9251947672983016,
 'recall': 0.8803458277361218,
 'support': 7359}
micro
{'f1-score': 0.9489061013724691,
 'precision': 0.9489061013724691,
 'recall': 0.9489061013724691,
 'support': 7359}
weighted
{'f1-score': 0.9482194234781568,
 'precision': 0.9485328602473018,
 'recall': 0.9489061013724691,
 'support': 7359}
### report_full
macro
{'f1-score': 0.7850659615538026,
 'precision': 0.9251947672983016,
 'recall': 0.7290135753813018,
 'support': 8154}
micro
{'f1-score': 0.9002771868755238,
 'precision': 0.9489061013724691,
 'recall': 0.8563895020848663,
 'support': 8154}
weighted
{'f1-score': 0.8891814928892925,
 'precision': 0.9456548960061023,
 'recall': 0.8563895020848663,
 'support': 8154}
```

## javascript
### Summary
27 rules, avg.len. 9.3

| | |
|-|-|
|Min support|92|
|Max support|3922|
|Min confidence|0.9251968264579773|
|Max confidence|0.9996517896652222|

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
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 143.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3922.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 1.000. Support: 1088.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 145.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 7<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 122.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.978. Support: 1188.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {VALUE}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.957. Support: 943.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 367.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 3258.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1796.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type = TemplateLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 111.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 296.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {BLOCK} and not in {QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 122.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {VARIABLE} and not in {BLOCK, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1436.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {BLOCK, QUALIFIED, VARIABLE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 881.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_col ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {BLOCK, OPERATOR, QUALIFIED, VARIABLE}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 509.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 152.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 592.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 490.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 337.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {LITERAL, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 242.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {LITERAL, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 92.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {EXPRESSION} and not in {LITERAL, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 737.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {EXPRESSION, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 210.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {EXPRESSION, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 200.` |
| 26 | `  -1.diff_col ≥ 1<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length = 0<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {EXPRESSION, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 127.` |
| 27 | `  -1.diff_col = 0<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {EXPRESSION, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 135.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.333333333333334, "max_conf": 0.9996517896652222, "max_support": 3922, "min_conf": 0.9251968264579773, "min_support": 92, "num_rules": 27}}
```
</details>

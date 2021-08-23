# Model report for file:///tmp/top-repos-quality-repos-7qobfxjh/pslab-desktop.git HEAD 7e8a5b1f8c3982219a57ba2dd4ab8e16758ea5b3

### Dump

```json
{'created_at': '2021-08-21 09:46:40',
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
 'size': '19.0 kB',
 'tags': [],
 'uuid': '9cd9a7e6-579d-4a88-8968-986955c5800c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7qobfxjh/pslab-desktop.git 7e8a5b1f8c3982219a57ba2dd4ab8e16758ea5b3

# javascript
43 rules, avg.len. 8.0
## train
PPCR: 0.945881
### report
macro
{'f1-score': 0.8106834852137735,
 'precision': 0.832497530098663,
 'recall': 0.792136291173517,
 'support': 46316}
micro
{'f1-score': 0.95150703860437,
 'precision': 0.95150703860437,
 'recall': 0.95150703860437,
 'support': 46316}
weighted
{'f1-score': 0.9502517261303886,
 'precision': 0.9504043177643534,
 'recall': 0.95150703860437,
 'support': 46316}
### report_full
macro
{'f1-score': 0.7476394713773532,
 'precision': 0.832497530098663,
 'recall': 0.6933851199926812,
 'support': 48966}
micro
{'f1-score': 0.9250435549211813,
 'precision': 0.95150703860437,
 'recall': 0.9000122534003185,
 'support': 48966}
weighted
{'f1-score': 0.9200204092127513,
 'precision': 0.9482244529650277,
 'recall': 0.9000122534003185,
 'support': 48966}
## test
PPCR: 0.944415
### report
macro
{'f1-score': 0.7888121619718667,
 'precision': 0.809966514328126,
 'recall': 0.7717424176565897,
 'support': 8903}
micro
{'f1-score': 0.9285634055936202,
 'precision': 0.9285634055936202,
 'recall': 0.9285634055936202,
 'support': 8903}
weighted
{'f1-score': 0.9263999730422845,
 'precision': 0.9267037793480662,
 'recall': 0.9285634055936202,
 'support': 8903}
### report_full
macro
{'f1-score': 0.7343011759341395,
 'precision': 0.809966514328126,
 'recall': 0.6891831401179918,
 'support': 9427}
micro
{'f1-score': 0.9020185488270595,
 'precision': 0.9285634055936202,
 'recall': 0.8769491885011138,
 'support': 9427}
weighted
{'f1-score': 0.8965210304813778,
 'precision': 0.9248488022445674,
 'recall': 0.8769491885011138,
 'support': 9427}
```

## javascript
### Summary
27 rules, avg.len. 7.9

| | |
|-|-|
|Min support|94|
|Max support|14395|
|Min confidence|0.9200116395950317|
|Max confidence|0.999280571937561|

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
| 1 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type = StringLiteral<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 251.` |
| 2 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≤ 19<br>	∧ -5.diff_offset ≥ 28<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 270.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.998. Support: 254.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.999. Support: 338.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 238.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = StringLiteral<br>	∧ -2.roles in {VALUE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 179.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≥ 9<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.990. Support: 153.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 8<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 278.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +5.reserved = ><br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 121.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +5.reserved not in {>}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 190.` |
| 11 | `  •••start_col ≥ 38<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label not in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 576.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {}}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {'}<br>	∧ -2.reserved not in {)}<br>	∧ -3.reserved not in {.}<br>	∧ -4.label not in {<+space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 14395.` |
| 13 | `  ^1.roles in {QUALIFIED} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2175.` |
| 14 | `  -1.label in {<space>}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 914.` |
| 15 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.987. Support: 866.` |
| 16 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 1719.` |
| 17 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 124.` |
| 18 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {:, {}<br>	∧ -5.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 561.` |
| 19 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 137.` |
| 20 | `  •••start_col ≤ 5<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.961. Support: 190.` |
| 21 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {,}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 132.` |
| 22 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 884.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 695.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≤ 13<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 543.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.length ≤ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.976. Support: 565.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = <<br>	∧ +3.internal_type = JSXIdentifier<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 114.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 94.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.851851851851852, "max_conf": 0.999280571937561, "max_support": 14395, "min_conf": 0.9200116395950317, "min_support": 94, "num_rules": 27}}
```
</details>

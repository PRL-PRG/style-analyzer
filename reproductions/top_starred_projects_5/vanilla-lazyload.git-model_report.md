# Model report for file:///tmp/top-repos-quality-repos-e4r9sw52/vanilla-lazyload.git HEAD 7b2d8b26bde0a6e03ed411b83f80c62b491fc81d

### Dump

```json
{'created_at': '2021-08-30 00:13:23',
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
 'size': '16.3 kB',
 'tags': [],
 'uuid': '6ac6b00f-063a-45f9-a004-bd36c1fc00f9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-e4r9sw52/vanilla-lazyload.git 7b2d8b26bde0a6e03ed411b83f80c62b491fc81d

# javascript
43 rules, avg.len. 6.3
## train
PPCR: 0.854635
### report
macro
{'f1-score': 0.39328477164582193,
 'precision': 0.3911705897079875,
 'recall': 0.39811479133453037,
 'support': 4868}
micro
{'f1-score': 0.8989317995069844,
 'precision': 0.8989317995069844,
 'recall': 0.8989317995069844,
 'support': 4868}
weighted
{'f1-score': 0.8823237181333766,
 'precision': 0.872685223087252,
 'recall': 0.8989317995069844,
 'support': 4868}
### report_full
macro
{'f1-score': 0.373968144016898,
 'precision': 0.3911705897079875,
 'recall': 0.3621065229370437,
 'support': 5696}
micro
{'f1-score': 0.8284740628549792,
 'precision': 0.8989317995069844,
 'recall': 0.7682584269662921,
 'support': 5696}
weighted
{'f1-score': 0.771239985558199,
 'precision': 0.7850296890513526,
 'recall': 0.7682584269662921,
 'support': 5696}
## test
PPCR: 0.826677
### report
macro
{'f1-score': 0.38062600855217277,
 'precision': 0.36800773657263897,
 'recall': 0.3969116178160611,
 'support': 1097}
micro
{'f1-score': 0.8505013673655424,
 'precision': 0.8505013673655424,
 'recall': 0.8505013673655424,
 'support': 1097}
weighted
{'f1-score': 0.8156526254813584,
 'precision': 0.7894591259495429,
 'recall': 0.8505013673655424,
 'support': 1097}
### report_full
macro
{'f1-score': 0.3446299320189719,
 'precision': 0.36800773657263897,
 'recall': 0.33070312500702403,
 'support': 1327}
micro
{'f1-score': 0.7698019801980198,
 'precision': 0.8505013673655424,
 'recall': 0.7030896759608138,
 'support': 1327}
weighted
{'f1-score': 0.7025673003923691,
 'precision': 0.7158637472607224,
 'recall': 0.7030896759608138,
 'support': 1327}
```

## javascript
### Summary
25 rules, avg.len. 5.7

| | |
|-|-|
|Min support|138|
|Max support|1566|
|Min confidence|0.9262499809265137|
|Max confidence|0.9986072182655334|

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
                     'max_depth': 10,
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
| 1 | `  -1.reserved not in {;}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 260.` |
| 2 | `  -1.reserved = .<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.979. Support: 211.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.997. Support: 154.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.953. Support: 1168.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.998. Support: 283.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>⇒ y = "<br>Confidence: 0.997. Support: 190.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 244.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.989. Support: 141.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION, INITIALIZATION}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 165.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 1536.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.982. Support: 138.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 374.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 1566.` |
| 14 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 152.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 359.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 228.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1096.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.926. Support: 400.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1418.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, ;}<br>⇒ y = "<br>Confidence: 0.997. Support: 174.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 169.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 243.` |
| 23 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = "<br>Confidence: 0.997. Support: 176.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 150.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1097.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.68, "max_conf": 0.9986072182655334, "max_support": 1566, "min_conf": 0.9262499809265137, "min_support": 138, "num_rules": 25}}
```
</details>

# Model report for file:///tmp/top-repos-quality-repos-kvmignaw/pikbooth.git HEAD 1136cc8a9cc9b3dac6f509f3bd00b514bc6db3a2

### Dump

```json
{'created_at': '2021-08-29 00:03:13',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.3 kB',
 'tags': [],
 'uuid': '2cd98c9b-cbf5-4bd3-81f0-043ae30d7d86',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kvmignaw/pikbooth.git 1136cc8a9cc9b3dac6f509f3bd00b514bc6db3a2

# javascript
23 rules, avg.len. 7.7
## train
PPCR: 0.918286
### report
macro
{'f1-score': 0.8322316414248587,
 'precision': 0.8341539260939843,
 'recall': 0.8322246221756762,
 'support': 32601}
micro
{'f1-score': 0.9764424404159382,
 'precision': 0.9764424404159382,
 'recall': 0.9764424404159382,
 'support': 32601}
weighted
{'f1-score': 0.9744767581085995,
 'precision': 0.9730044588822445,
 'recall': 0.9764424404159382,
 'support': 32601}
### report_full
macro
{'f1-score': 0.7255478889137832,
 'precision': 0.8341539260939843,
 'recall': 0.6760101118961203,
 'support': 35502}
micro
{'f1-score': 0.9348486850799524,
 'precision': 0.9764424404159382,
 'recall': 0.8966537096501606,
 'support': 35502}
weighted
{'f1-score': 0.9201538495576932,
 'precision': 0.9639006472689714,
 'recall': 0.8966537096501606,
 'support': 35502}
## test
PPCR: 0.898892
### report
macro
{'f1-score': 0.5410923452762071,
 'precision': 0.5243298171878158,
 'recall': 0.5755910681884818,
 'support': 2676}
micro
{'f1-score': 0.8221225710014948,
 'precision': 0.8221225710014948,
 'recall': 0.8221225710014948,
 'support': 2676}
weighted
{'f1-score': 0.8091284134980633,
 'precision': 0.8067541404902648,
 'recall': 0.8221225710014948,
 'support': 2676}
### report_full
macro
{'f1-score': 0.49561587935434137,
 'precision': 0.5243298171878158,
 'recall': 0.5218984459897018,
 'support': 2977}
micro
{'f1-score': 0.7783477799398549,
 'precision': 0.8221225710014948,
 'recall': 0.7389989922741015,
 'support': 2977}
weighted
{'f1-score': 0.7415179513472367,
 'precision': 0.77495707656572,
 'recall': 0.7389989922741015,
 'support': 2977}
```

## javascript
### Summary
18 rules, avg.len. 7.2

| | |
|-|-|
|Min support|105|
|Max support|6390|
|Min confidence|0.9211956262588501|
|Max confidence|0.9997433423995972|

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
                     'min_samples_split': 190,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 6390.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 105.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 2918.` |
| 4 | `  -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.989. Support: 868.` |
| 5 | `  -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.951. Support: 883.` |
| 6 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 612.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.921. Support: 552.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 4298.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1948.` |
| 10 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1082.` |
| 11 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, if, }}<br>	∧ +5.reserved = function<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.958. Support: 107.` |
| 12 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, if, }}<br>	∧ +4.reserved = =<br>	∧ +5.reserved not in {function}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 127.` |
| 13 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 401.` |
| 14 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 194.` |
| 15 | `  -1.diff_offset ≥ 103<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 118.` |
| 16 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 295.` |
| 17 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 236.` |
| 18 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 3930.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.222222222222222, "max_conf": 0.9997433423995972, "max_support": 6390, "min_conf": 0.9211956262588501, "min_support": 105, "num_rules": 18}}
```
</details>

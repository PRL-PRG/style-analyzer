# Model report for file:///tmp/top-repos-quality-repos-kli0n8xe/sei-project-04.git HEAD dfb3484a5f1667d1855d6413dc40b0fae7362bbc

### Dump

```json
{'created_at': '2021-08-21 18:24:18',
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
 'size': '16.1 kB',
 'tags': [],
 'uuid': '4f08915a-ecd1-4c32-90a1-1ca9b68b0593',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kli0n8xe/sei-project-04.git dfb3484a5f1667d1855d6413dc40b0fae7362bbc

# javascript
36 rules, avg.len. 6.9
## train
PPCR: 0.869379
### report
macro
{'f1-score': 0.4823408779371854,
 'precision': 0.5071007712695269,
 'recall': 0.46771509968236874,
 'support': 9531}
micro
{'f1-score': 0.8772426817752595,
 'precision': 0.8772426817752597,
 'recall': 0.8772426817752597,
 'support': 9531}
weighted
{'f1-score': 0.8554376516680549,
 'precision': 0.8457554409564568,
 'recall': 0.8772426817752597,
 'support': 9531}
### report_full
macro
{'f1-score': 0.46277061939329067,
 'precision': 0.5071007712695269,
 'recall': 0.4375951396054096,
 'support': 10963}
micro
{'f1-score': 0.8159461305748025,
 'precision': 0.8772426817752597,
 'recall': 0.7626562072425431,
 'support': 10963}
weighted
{'f1-score': 0.7694542864939161,
 'precision': 0.7997272461637619,
 'recall': 0.7626562072425431,
 'support': 10963}
## test
PPCR: 0.854104
### report
macro
{'f1-score': 0.4774724856608197,
 'precision': 0.507460871248419,
 'recall': 0.4596705967955887,
 'support': 2289}
micro
{'f1-score': 0.892092616863259,
 'precision': 0.892092616863259,
 'recall': 0.892092616863259,
 'support': 2289}
weighted
{'f1-score': 0.8727000703117234,
 'precision': 0.8629669484466602,
 'recall': 0.892092616863259,
 'support': 2289}
### report_full
macro
{'f1-score': 0.4558782329726038,
 'precision': 0.507460871248419,
 'recall': 0.4274975522262485,
 'support': 2680}
micro
{'f1-score': 0.8218957536727711,
 'precision': 0.892092616863259,
 'recall': 0.7619402985074627,
 'support': 2680}
weighted
{'f1-score': 0.7719215427739123,
 'precision': 0.804818105780298,
 'recall': 0.7619402985074627,
 'support': 2680}
```

## javascript
### Summary
17 rules, avg.len. 5.9

| | |
|-|-|
|Min support|133|
|Max support|2446|
|Min confidence|0.9225663542747498|
|Max confidence|0.9988636374473572|

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
                     'min_samples_leaf': 90,
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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 440.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.974. Support: 408.` |
| 3 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 267.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 2446.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1391.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {INCOMPLETE} and not in {BLOCK, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 2397.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED} and not in {BLOCK, DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1339.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {BLOCK, DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.936. Support: 133.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {BLOCK, DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1402.` |
| 10 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -4.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 226.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.length ≥ 2<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.956. Support: 424.` |
| 12 | `  -1.diff_offset ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 139.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 260.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 1394.` |
| 15 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 192.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, CALL, DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 170.` |
| 17 | `  -1.diff_offset ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type = File<br>⇒ y = ␣<br>Confidence: 0.960. Support: 161.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.882352941176471, "max_conf": 0.9988636374473572, "max_support": 2446, "min_conf": 0.9225663542747498, "min_support": 133, "num_rules": 17}}
```
</details>

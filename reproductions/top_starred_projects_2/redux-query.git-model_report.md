# Model report for file:///tmp/top-repos-quality-repos-kzk3e2wu/redux-query.git HEAD 0687bf301dad8d78576ba1c7a0dcc39762651b1c

### Dump

```json
{'created_at': '2021-08-30 03:28:41',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.3 kB',
 'tags': [],
 'uuid': 'dd711661-12f4-4edd-80db-2893d778bf29',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kzk3e2wu/redux-query.git 0687bf301dad8d78576ba1c7a0dcc39762651b1c

# javascript
22 rules, avg.len. 6.0
## train
PPCR: 0.893329
### report
macro
{'f1-score': 0.7896205015053122,
 'precision': 0.8098174334712078,
 'recall': 0.7728229477786839,
 'support': 17235}
micro
{'f1-score': 0.934029590948651,
 'precision': 0.934029590948651,
 'recall': 0.934029590948651,
 'support': 17235}
weighted
{'f1-score': 0.932067732073363,
 'precision': 0.9312086688441528,
 'recall': 0.934029590948651,
 'support': 17235}
### report_full
macro
{'f1-score': 0.7229787568630088,
 'precision': 0.8098174334712078,
 'recall': 0.6624826562869459,
 'support': 19293}
micro
{'f1-score': 0.8814060446780552,
 'precision': 0.934029590948651,
 'recall': 0.8343958948841549,
 'support': 19293}
weighted
{'f1-score': 0.8752818454234614,
 'precision': 0.925808195667794,
 'recall': 0.8343958948841549,
 'support': 19293}
## test
PPCR: 0.835459
### report
macro
{'f1-score': 0.7584613136884921,
 'precision': 0.7687418288315431,
 'recall': 0.7526013639638179,
 'support': 3407}
micro
{'f1-score': 0.9131200469621368,
 'precision': 0.9131200469621368,
 'recall': 0.9131200469621368,
 'support': 3407}
weighted
{'f1-score': 0.9115987898877486,
 'precision': 0.9119661848655789,
 'recall': 0.9131200469621368,
 'support': 3407}
### report_full
macro
{'f1-score': 0.6605217444746652,
 'precision': 0.7687418288315431,
 'recall': 0.5948501374909806,
 'support': 4078}
micro
{'f1-score': 0.8312625250501003,
 'precision': 0.9131200469621368,
 'recall': 0.762873957822462,
 'support': 4078}
weighted
{'f1-score': 0.8218867302464533,
 'precision': 0.9061479282476147,
 'recall': 0.762873957822462,
 'support': 4078}
```

## javascript
### Summary
11 rules, avg.len. 5.4

| | |
|-|-|
|Min support|104|
|Max support|3092|
|Min confidence|0.9211026430130005|
|Max confidence|0.9993749856948853|

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
| 1 | `  -1.reserved = {<br>	∧ -3.length ≤ 1<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.921. Support: 526.` |
| 2 | `  -1.reserved = ;<br>	∧ +1.length ≤ 4<br>⇒ y = ⏎⏎<br>Confidence: 0.925. Support: 153.` |
| 3 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.998. Support: 285.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER} and not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 479.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = '<br>Confidence: 0.995. Support: 107.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 497.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 226.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 104.` |
| 9 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 800.` |
| 10 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.941. Support: 604.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3092.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.363636363636363, "max_conf": 0.9993749856948853, "max_support": 3092, "min_conf": 0.9211026430130005, "min_support": 104, "num_rules": 11}}
```
</details>

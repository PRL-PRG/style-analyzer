# Model report for file:///tmp/top-repos-quality-repos-zlgsufzr/artillery.git HEAD d3cb703f2720a3902a0d1af2454717b6e5a41074

### Dump

```json
{'created_at': '2021-08-30 04:16:46',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': '44cb6a50-5a9c-4c05-b4a8-1101f2f7551d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-zlgsufzr/artillery.git d3cb703f2720a3902a0d1af2454717b6e5a41074

# javascript
19 rules, avg.len. 5.9
## train
PPCR: 0.914926
### report
macro
{'f1-score': 0.8597278463550693,
 'precision': 0.9213012553336937,
 'recall': 0.8268233843316403,
 'support': 34963}
micro
{'f1-score': 0.9448273889540372,
 'precision': 0.9448273889540372,
 'recall': 0.9448273889540372,
 'support': 34963}
weighted
{'f1-score': 0.9432725210326911,
 'precision': 0.9459236515877076,
 'recall': 0.9448273889540372,
 'support': 34963}
### report_full
macro
{'f1-score': 0.7216584062536961,
 'precision': 0.9213012553336937,
 'recall': 0.6658866354532532,
 'support': 38214}
micro
{'f1-score': 0.9028519890129413,
 'precision': 0.9448273889540372,
 'recall': 0.8644475846548385,
 'support': 38214}
weighted
{'f1-score': 0.8830122917208669,
 'precision': 0.942399653986748,
 'recall': 0.8644475846548385,
 'support': 38214}
## test
PPCR: 0.909853
### report
macro
{'f1-score': 0.7990597786902713,
 'precision': 0.8705717500095219,
 'recall': 0.7631083600856415,
 'support': 9225}
micro
{'f1-score': 0.9409214092140922,
 'precision': 0.9409214092140922,
 'recall': 0.9409214092140922,
 'support': 9225}
weighted
{'f1-score': 0.9392474709902267,
 'precision': 0.9423490811969323,
 'recall': 0.9409214092140922,
 'support': 9225}
### report_full
macro
{'f1-score': 0.6659029926100037,
 'precision': 0.8705717500095219,
 'recall': 0.6220460580973738,
 'support': 10139}
micro
{'f1-score': 0.8965089857467465,
 'precision': 0.9409214092140922,
 'recall': 0.8561002071210179,
 'support': 10139}
weighted
{'f1-score': 0.8715014133532467,
 'precision': 0.9321224548801444,
 'recall': 0.8561002071210179,
 'support': 10139}
```

## javascript
### Summary
14 rules, avg.len. 5.7

| | |
|-|-|
|Min support|172|
|Max support|8858|
|Min confidence|0.9213286638259888|
|Max confidence|0.9995187520980835|

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
                     'min_samples_leaf': 120,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.983. Support: 5670.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.995. Support: 551.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1419.` |
| 4 | `  -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 749.` |
| 5 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 358.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 286.` |
| 7 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.922. Support: 223.` |
| 8 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 198.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 1039.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1101.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 172.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 259.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {RIGHT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 177.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {RIGHT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 8858.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.714285714285714, "max_conf": 0.9995187520980835, "max_support": 8858, "min_conf": 0.9213286638259888, "min_support": 172, "num_rules": 14}}
```
</details>

# Model report for file:///tmp/top-repos-quality-repos-ws6oywd0/newsfeed.git HEAD a69a02052c120132eb50c8ecb93ca15c6b2fc081

### Dump

```json
{'created_at': '2021-08-24 22:46:12',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': 'c644bc9d-a99c-47af-90b8-ee5d4c15a0c3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ws6oywd0/newsfeed.git a69a02052c120132eb50c8ecb93ca15c6b2fc081

# javascript
9 rules, avg.len. 4.8
## train
PPCR: 0.615764
### report
macro
{'f1-score': 0.5735337087185647,
 'precision': 0.6014689054200362,
 'recall': 0.5513334057048674,
 'support': 3875}
micro
{'f1-score': 0.9450322580645162,
 'precision': 0.9450322580645162,
 'recall': 0.9450322580645162,
 'support': 3875}
weighted
{'f1-score': 0.9405983826576828,
 'precision': 0.9412812245799334,
 'recall': 0.9450322580645162,
 'support': 3875}
### report_full
macro
{'f1-score': 0.42396999785456463,
 'precision': 0.6014689054200362,
 'recall': 0.33948054773367486,
 'support': 6293}
micro
{'f1-score': 0.7202989771833203,
 'precision': 0.9450322580645162,
 'recall': 0.5819164150643572,
 'support': 6293}
weighted
{'f1-score': 0.6751094539087684,
 'precision': 0.8641006171154277,
 'recall': 0.5819164150643572,
 'support': 6293}
## test
PPCR: 0.638629
### report
macro
{'f1-score': 0.5784553511873943,
 'precision': 0.6026732838435233,
 'recall': 0.5586628466425524,
 'support': 820}
micro
{'f1-score': 0.9536585365853658,
 'precision': 0.9536585365853658,
 'recall': 0.9536585365853658,
 'support': 820}
weighted
{'f1-score': 0.9497385141662532,
 'precision': 0.9495376859682352,
 'recall': 0.9536585365853658,
 'support': 820}
### report_full
macro
{'f1-score': 0.44048221696260126,
 'precision': 0.6026732838435233,
 'recall': 0.3577105586546915,
 'support': 1284}
micro
{'f1-score': 0.7433460076045626,
 'precision': 0.9536585365853658,
 'recall': 0.6090342679127726,
 'support': 1284}
weighted
{'f1-score': 0.7034596419575277,
 'precision': 0.8765934323493317,
 'recall': 0.6090342679127726,
 'support': 1284}
```

## javascript
### Summary
7 rules, avg.len. 4.3

| | |
|-|-|
|Min support|96|
|Max support|984|
|Min confidence|0.9421707987785339|
|Max confidence|0.9963768124580383|

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
| 1 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.963. Support: 984.` |
| 2 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.942. Support: 562.` |
| 3 | `  •••start_col ≤ 29<br>	∧ -1.length ≥ 3<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 386.` |
| 4 | `  -1.length ≤ 2<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = "<br>Confidence: 0.996. Support: 122.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = '<br>Confidence: 0.996. Support: 138.` |
| 6 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.953. Support: 96.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 108.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.285714285714286, "max_conf": 0.9963768124580383, "max_support": 984, "min_conf": 0.9421707987785339, "min_support": 96, "num_rules": 7}}
```
</details>

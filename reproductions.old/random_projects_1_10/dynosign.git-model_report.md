# Model report for file:///tmp/top-repos-quality-repos-wsmbqhds/dynosign.git HEAD 0337f13569a3ac2e28372bc79089478e69c1bbeb

### Dump

```json
{'created_at': '2021-08-22 13:48:31',
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
 'size': '16.0 kB',
 'tags': [],
 'uuid': 'd2a52efa-92fc-4836-af2d-dbbd84289d4f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-wsmbqhds/dynosign.git 0337f13569a3ac2e28372bc79089478e69c1bbeb

# javascript
16 rules, avg.len. 5.4
## train
PPCR: 0.873264
### report
macro
{'f1-score': 0.6312994636422121,
 'precision': 0.6444390182063089,
 'recall': 0.619859559207244,
 'support': 10122}
micro
{'f1-score': 0.927188302706975,
 'precision': 0.9271883027069749,
 'recall': 0.9271883027069749,
 'support': 10122}
weighted
{'f1-score': 0.9205418271712149,
 'precision': 0.9148532048975858,
 'recall': 0.9271883027069749,
 'support': 10122}
### report_full
macro
{'f1-score': 0.5782242472339275,
 'precision': 0.6444390182063089,
 'recall': 0.5315124878481876,
 'support': 11591}
micro
{'f1-score': 0.8644590798139362,
 'precision': 0.9271883027069749,
 'recall': 0.8096799240790268,
 'support': 11591}
weighted
{'f1-score': 0.8426930573535446,
 'precision': 0.8847556688317278,
 'recall': 0.8096799240790268,
 'support': 11591}
## test
PPCR: 0.834449
### report
macro
{'f1-score': 0.5998495587107886,
 'precision': 0.6193520998195838,
 'recall': 0.5853199004532359,
 'support': 2243}
micro
{'f1-score': 0.8943379402585823,
 'precision': 0.8943379402585823,
 'recall': 0.8943379402585823,
 'support': 2243}
weighted
{'f1-score': 0.8814724220972546,
 'precision': 0.8709263590930677,
 'recall': 0.8943379402585823,
 'support': 2243}
### report_full
macro
{'f1-score': 0.5490820399686427,
 'precision': 0.6193520998195838,
 'recall': 0.5073869344061661,
 'support': 2688}
micro
{'f1-score': 0.8136280673291422,
 'precision': 0.8943379402585823,
 'recall': 0.7462797619047619,
 'support': 2688}
weighted
{'f1-score': 0.7771720494006338,
 'precision': 0.8264636398291281,
 'recall': 0.7462797619047619,
 'support': 2688}
```

## javascript
### Summary
8 rules, avg.len. 4.2

| | |
|-|-|
|Min support|113|
|Max support|1472|
|Min confidence|0.9551971554756165|
|Max confidence|0.9986876845359802|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1472.` |
| 2 | `  -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 149.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 302.` |
| 4 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 201.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 381.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 279.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 113.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 122.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.25, "max_conf": 0.9986876845359802, "max_support": 1472, "min_conf": 0.9551971554756165, "min_support": 113, "num_rules": 8}}
```
</details>

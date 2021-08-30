# Model report for file:///tmp/top-repos-quality-repos-o1z9cpbc/kanmail.git HEAD 98699a14fa32aa1fd6d7384328ca30da6aae7a01

### Dump

```json
{'created_at': '2021-08-30 01:15:30',
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
 'uuid': '7a9e7fe7-6366-4a5d-9ebf-8f8c3462a354',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-o1z9cpbc/kanmail.git 98699a14fa32aa1fd6d7384328ca30da6aae7a01

# javascript
14 rules, avg.len. 6.5
## train
PPCR: 0.825122
### report
macro
{'f1-score': 0.7686579072071759,
 'precision': 0.7996092020360406,
 'recall': 0.7455111002311048,
 'support': 9847}
micro
{'f1-score': 0.9444500863207068,
 'precision': 0.9444500863207068,
 'recall': 0.9444500863207068,
 'support': 9847}
weighted
{'f1-score': 0.9434797030394304,
 'precision': 0.9451578473163349,
 'recall': 0.9444500863207068,
 'support': 9847}
### report_full
macro
{'f1-score': 0.6583932098113239,
 'precision': 0.7996092020360406,
 'recall': 0.5897611206868815,
 'support': 11934}
micro
{'f1-score': 0.8539552821266242,
 'precision': 0.9444500863207068,
 'recall': 0.7792860734037205,
 'support': 11934}
weighted
{'f1-score': 0.8302904222731855,
 'precision': 0.9080991389909326,
 'recall': 0.7792860734037205,
 'support': 11934}
## test
PPCR: 0.836542
### report
macro
{'f1-score': 0.7418716527209325,
 'precision': 0.8143779978908664,
 'recall': 0.6941098135332552,
 'support': 2216}
micro
{'f1-score': 0.9485559566787004,
 'precision': 0.9485559566787004,
 'recall': 0.9485559566787004,
 'support': 2216}
weighted
{'f1-score': 0.9461172951498773,
 'precision': 0.9501598978390636,
 'recall': 0.9485559566787004,
 'support': 2216}
### report_full
macro
{'f1-score': 0.6426747320253504,
 'precision': 0.8143779978908664,
 'recall': 0.578418918060195,
 'support': 2649}
micro
{'f1-score': 0.8641315519013362,
 'precision': 0.9485559566787004,
 'recall': 0.7935069837674594,
 'support': 2649}
weighted
{'f1-score': 0.8320058167386954,
 'precision': 0.9110312167721024,
 'recall': 0.7935069837674594,
 'support': 2649}
```

## javascript
### Summary
8 rules, avg.len. 5.8

| | |
|-|-|
|Min support|105|
|Max support|1771|
|Min confidence|0.9631295204162598|
|Max confidence|0.9986772537231445|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1771.` |
| 2 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.998. Support: 233.` |
| 3 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 402.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 208.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 1668.` |
| 6 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 378.` |
| 7 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.996. Support: 127.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 105.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.75, "max_conf": 0.9986772537231445, "max_support": 1771, "min_conf": 0.9631295204162598, "min_support": 105, "num_rules": 8}}
```
</details>

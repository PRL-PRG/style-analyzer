# Model report for file:///tmp/top-repos-quality-repos-uciyj34o/oim-cms.git HEAD 7918ec7ed93a536669d7c17312aacbeec0cdfbe3

### Dump

```json
{'created_at': '2021-08-21 05:19:58',
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
 'size': '14.6 kB',
 'tags': [],
 'uuid': '3f6c74c9-4628-40c2-b72b-b8f8577feaee',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-uciyj34o/oim-cms.git 7918ec7ed93a536669d7c17312aacbeec0cdfbe3

# javascript
22 rules, avg.len. 4.8
## train
PPCR: 0.584946
### report
macro
{'f1-score': 0.28585021084171036,
 'precision': 0.2862385818437373,
 'recall': 0.28604366577048174,
 'support': 2207}
micro
{'f1-score': 0.8785681921159946,
 'precision': 0.8785681921159946,
 'recall': 0.8785681921159946,
 'support': 2207}
weighted
{'f1-score': 0.8647972675021476,
 'precision': 0.8526426818577068,
 'recall': 0.8785681921159946,
 'support': 2207}
### report_full
macro
{'f1-score': 0.2338514253680546,
 'precision': 0.2862385818437373,
 'recall': 0.20208233183053778,
 'support': 3773}
micro
{'f1-score': 0.648494983277592,
 'precision': 0.8785681921159946,
 'recall': 0.5139146567717996,
 'support': 3773}
weighted
{'f1-score': 0.5770455426088967,
 'precision': 0.6733305490043754,
 'recall': 0.5139146567717996,
 'support': 3773}
## test
PPCR: 0.670010
### report
macro
{'f1-score': 0.2622804522987338,
 'precision': 0.2693785251541027,
 'recall': 0.2579063883617963,
 'support': 668}
micro
{'f1-score': 0.8607784431137725,
 'precision': 0.8607784431137725,
 'recall': 0.8607784431137725,
 'support': 668}
weighted
{'f1-score': 0.8446801582696885,
 'precision': 0.8330951643781429,
 'recall': 0.8607784431137725,
 'support': 668}
### report_full
macro
{'f1-score': 0.22077739727276371,
 'precision': 0.2693785251541027,
 'recall': 0.1942587398057244,
 'support': 997}
micro
{'f1-score': 0.6906906906906907,
 'precision': 0.8607784431137725,
 'recall': 0.5767301905717152,
 'support': 997}
weighted
{'f1-score': 0.6247115069208984,
 'precision': 0.7015988507194799,
 'recall': 0.5767301905717152,
 'support': 997}
```

## javascript
### Summary
15 rules, avg.len. 4.9

| | |
|-|-|
|Min support|172|
|Max support|983|
|Min confidence|0.9292980432510376|
|Max confidence|0.9983922839164734|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 983.` |
| 2 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 680.` |
| 3 | `  -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 311.` |
| 4 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 172.` |
| 5 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.949. Support: 600.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 185.` |
| 7 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 598.` |
| 8 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 193.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 374.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 194.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 201.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 181.` |
| 13 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 616.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 15 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 208.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.866666666666666, "max_conf": 0.9983922839164734, "max_support": 983, "min_conf": 0.9292980432510376, "min_support": 172, "num_rules": 15}}
```
</details>
